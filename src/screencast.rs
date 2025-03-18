use crate::common::{CursorMode, PersistMode, SourceType};
use crate::errors::{Error, Result};
use crate::proxy::request::ResponseStream;
use crate::proxy::screencast::{
  ZCreateSessionReq, ZCreateSessionRes, ZScreenCastProxy, ZSelectSourcesReq, ZStartReq, ZStartRes,
  ZStartStream,
};
use crate::request::RequestPortal;
use std::collections::HashMap;
use zbus::Connection;
use zbus::export::ordered_stream::OrderedStreamExt;
use zvariant::OwnedFd;

/// https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.portal.ScreenCast.xml

/// portal for to create screencast sessions.
pub struct ScreencastPortal {
  proxy: ZScreenCastProxy<'static>,
  handle_token: String,
  session_token: String,
  responses: ResponseStream,
}

impl ScreencastPortal {
  /// create ScreencastPortal instance
  ///
  /// `handle_token`: string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `session_token`: string that will be used as the last element of the session handle. Must be
  /// a valid object path element. See the :ref:`org.freedesktop.portal.Session` documentation for
  /// more information about the session handle
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(
    handle_token: &str,
    session_token: &str,
    connection: Connection,
  ) -> Result<ScreencastPortal> {
    let proxy = ZScreenCastProxy::new(&connection).await?;
    let responses = RequestPortal::new(handle_token, connection)
      .await?
      .responses()
      .await?;
    let portal = ScreencastPortal {
      proxy,
      handle_token: handle_token.to_string(),
      session_token: session_token.to_string(),
      responses,
    };
    Ok(portal)
  }

  /// create screencast session and open pipewire remote
  pub async fn screencast(&mut self, req: ScreencastReq) -> Result<ScreencastRes> {
    let create_session_req =
      ZCreateSessionReq::new(self.handle_token.as_str(), self.session_token.as_str());
    let _ = self.proxy.create_session(&create_session_req).await;
    let session = self
      .responses
      .next()
      .await
      .ok_or(Error::SignalStreamClosed)?
      .args::<ZCreateSessionRes>()?
      .results;
    let session = session.session_handle()?;
    let select_source_req = ZSelectSourcesReq::new(self.handle_token.as_str())
      .types(req.source_type.map(|t| t.bits()))
      .multiple(req.multiple)
      .cursor_mode(req.cursor_mode.map(|c| c.bits()))
      .restore_token(req.restore_token.as_deref())
      .persist_mode(req.persist_mode.map(|p| p.bits()));
    let _ = self
      .proxy
      .select_sources(session.as_ref(), &select_source_req)
      .await?;
    let _ = self
      .responses
      .next()
      .await
      .ok_or(Error::SignalStreamClosed)?;
    let start_select_req = ZStartReq::new(self.handle_token.as_str());
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let _ = self
      .proxy
      .start(session.as_ref(), parent_window, &start_select_req)
      .await?;
    let source = self
      .responses
      .next()
      .await
      .ok_or(Error::SignalStreamClosed)?
      .args::<ZStartRes>()?
      .results;
    let fd = self
      .proxy
      .open_pipe_wire_remote(session.as_ref(), HashMap::new())
      .await?;
    Ok(create_screencast_res(fd, source))
  }
}

/// request of [`ScreencastPortal::screencast`]
#[derive(Debug)]
pub struct ScreencastReq {
  pub(crate) multiple: Option<bool>,
  pub(crate) source_type: Option<SourceType>,
  pub(crate) cursor_mode: Option<CursorMode>,
  pub(crate) persist_mode: Option<PersistMode>,
  pub(crate) restore_token: Option<String>,
  pub(crate) parent_window: Option<String>,
}

impl ScreencastReq {
  pub fn new() -> ScreencastReq {
    ScreencastReq {
      multiple: None,
      source_type: None,
      cursor_mode: None,
      persist_mode: None,
      restore_token: None,
      parent_window: None,
    }
  }

  pub fn multiple(mut self, multiple: bool) -> Self {
    self.multiple = Some(multiple);
    self
  }

  pub fn source_type(mut self, source_type: SourceType) -> Self {
    self.source_type = Some(source_type);
    self
  }

  pub fn cursor_mode(mut self, cursor_mode: CursorMode) -> Self {
    self.cursor_mode = Some(cursor_mode);
    self
  }

  pub fn persist_mode(mut self, persist_mode: PersistMode) -> Self {
    self.persist_mode = Some(persist_mode);
    self
  }

  pub fn restore_token(mut self, restore_token: &str) -> Self {
    self.restore_token = Some(restore_token.to_string());
    self
  }

  pub fn parent_window(mut self, parent_windoe: &str) -> Self {
    self.parent_window = Some(parent_windoe.to_string());
    self
  }
}

/// response of [`ScreencastPortal::screencast`]
#[derive(Debug)]
pub struct ScreencastRes {
  pub streams: Vec<SelectedSource>,
  pub fd: std::os::fd::OwnedFd,
}

#[derive(Debug)]
pub struct SelectedSource {
  pub node_id: u32,
  pub identifier: Option<String>,
  pub source_type: SourceType,
  pub size: Option<(i32, i32)>,
  pub position: Option<(i32, i32)>,
  pub mapping_id: Option<String>,
}

impl From<(u32, ZStartStream)> for SelectedSource {
  fn from(value: (u32, ZStartStream)) -> Self {
    let ZStartStream {
      id,
      position,
      size,
      source_type,
      mapping_id,
    } = value.1;
    SelectedSource {
      node_id: value.0,
      identifier: id,
      source_type: SourceType::from_bits(source_type).unwrap_or(SourceType::Window),
      size,
      position,
      mapping_id,
    }
  }
}

fn create_screencast_res(fd: OwnedFd, res: ZStartRes) -> ScreencastRes {
  let streams = res
    .streams
    .into_iter()
    .map(|stream| SelectedSource::from(stream))
    .collect();
  ScreencastRes {
    fd: fd.into(),
    streams,
  }
}
