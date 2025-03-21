use zbus::Connection;
use zbus::export::ordered_stream::OrderedStreamExt;
use crate::proxy::request::ResponseStream;
use crate::proxy::screenshot::{ZPickColorReq, ZPickColorRes, ZScreenshotProxy, ZScreenshotReq, ZScreenshotRes};
use crate::errors::{Result, Error};
use crate::request::RequestPortal;

pub struct ScreenshotPortal {
  handle_token: String,
  proxy: ZScreenshotProxy<'static>,
  signals: ResponseStream,
}

impl ScreenshotPortal {
  /// create ScreenshotPortal instance
  ///
  /// `handle_token`: string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(handle_token: String, connection: Connection) -> Result<Self> {
    let proxy = ZScreenshotProxy::new(&connection).await?;
    let signals = RequestPortal::new(&handle_token, connection)
      .await?
      .responses()
      .await?;
    let portal = ScreenshotPortal {
      handle_token,
      proxy,
      signals
    };
    Ok(portal)
  }

  /// takes a screenshot
  pub async fn screenshot(&mut self, req: ScreenshotReq) -> Result<ScreenshotRes> {
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let req = ZScreenshotReq::new(self.handle_token.as_str())
      .modal(req.modal)
      .interactive(req.interactive);
    let _ = self.proxy.screenshot(parent_window, &req).await;
    let res = self.signals.next().await
      .ok_or(Error::SignalStreamClosed)?
      .args::<ZScreenshotRes>()?
      .results;
    Ok(ScreenshotRes::from(res))
  }

  /// obtains the color of a single pixel
  pub async fn pick_color(&mut self, req: PickColorReq) -> Result<PickColorRes> {
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let req = ZPickColorReq::new(self.handle_token.as_str());
    let _ = self.proxy.pick_color(parent_window, &req).await;
    let res = self.signals.next().await
    .ok_or(Error::SignalStreamClosed)?
    .args::<ZPickColorRes>()?
    .results;
    Ok(PickColorRes::from(res))
  }
}

/// request of [`ScreenshotPortal::screenshot`]
#[derive(Debug)]
pub struct ScreenshotReq {
  parent_window: Option<String>,
  modal: Option<bool>,
  interactive: Option<bool>,
}

impl ScreenshotReq {
  pub fn new() -> ScreenshotReq {
    ScreenshotReq {
      parent_window: None,
      modal: None,
      interactive: None,
    }
  }

  pub fn modal(mut self, modal: bool) -> ScreenshotReq {
    self.modal = Some(modal);
    self
  }

  pub fn interactive(mut self, interactive: bool) -> ScreenshotReq {
    self.interactive = Some(interactive);
    self
  }
}

/// response of [`ScreenshotPortal::screenshot`]
#[derive(Debug)]
pub struct ScreenshotRes {
  pub uri: String,
}

impl From<ZScreenshotRes> for ScreenshotRes {
  fn from(value: ZScreenshotRes) -> Self {
    let ZScreenshotRes { uri } = value;
    Self { uri }
  }
}

/// request of [`ScreenshotPortal::pick_color`]
#[derive(Debug)]
pub struct PickColorReq {
  parent_window: Option<String>,
}

impl PickColorReq {
  pub fn new() -> Self {
    Self { parent_window: None }
  }

  pub fn parent_window(&mut self, parent_window: String) -> &mut Self {
    self.parent_window = Some(parent_window);
    self
  }
}

/// response of [`ScreenshotPortal::pick_color`]
#[derive(Debug)]
pub struct PickColorRes {
  pub r: f64,
  pub g: f64,
  pub b: f64,
}

impl From<ZPickColorRes> for PickColorRes {
  fn from(value: ZPickColorRes) -> Self {
    let ZPickColorRes { color: (r, g, b) } = value;
    Self { r, g, b }
  }
}