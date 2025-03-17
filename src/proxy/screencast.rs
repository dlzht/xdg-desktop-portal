use std::collections::HashMap;
use crate::errors::Result;
use zbus::proxy;
use zvariant::{DeserializeDict, ObjectPath, OwnedFd, OwnedObjectPath, SerializeDict, Type, Value};

#[proxy(
  interface = "org.freedesktop.portal.ScreenCast",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZScreenCast {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  #[zbus(property, name = "AvailableCursorModes")]
  fn available_cursor_modes(&self) -> Result<u32>;

  #[zbus(property, name = "AvailableSourceTypes")]
  fn available_source_types(&self) -> Result<u32>;

  fn create_session(&self, options: &ZCreateSessionReq) -> Result<OwnedObjectPath>;

  fn select_sources(
    &self,
    session_handle: ObjectPath<'_>,
    options: &ZSelectSourcesReq,
  ) -> Result<OwnedObjectPath>;

  fn start(
    &self,
    session_handle: ObjectPath<'_>,
    parent_window: &str,
    options: &ZStartReq,
  ) -> Result<OwnedObjectPath>;

  fn open_pipe_wire_remote(&self, session_handle: ObjectPath<'_>, options: HashMap<String, Value<'_>>) -> Result<OwnedFd>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZCreateSessionReq {
  pub(crate) handle_token: String,
  pub(crate) session_handle_token: String,
}

impl ZCreateSessionReq {
  pub fn new(handle_token: &str, session_handle_token: &str) -> ZCreateSessionReq {
    ZCreateSessionReq {
      handle_token: handle_token.to_string(),
      session_handle_token: session_handle_token.to_string(),
    }
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZCreateSessionRes {
  session_handle: String,
}

impl ZCreateSessionRes {
  pub fn session_handle(&self) -> Result<OwnedObjectPath> {
    Ok(OwnedObjectPath::try_from(self.session_handle.as_str())?)
  }
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZSelectSourcesReq {
  handle_token: String,
  types: Option<u32>,
  multiple: Option<bool>,
  cursor_mode: Option<u32>,
  restore_token: Option<String>,
  persist_mode: Option<u32>,
}

impl ZSelectSourcesReq {
  pub fn new(handle_token: &str) -> ZSelectSourcesReq {
    ZSelectSourcesReq {
      handle_token: handle_token.to_string(),
      types: None,
      multiple: None,
      cursor_mode: None,
      restore_token: None,
      persist_mode: None,
    }
  }

  pub fn types(mut self, types: Option<u32>) -> ZSelectSourcesReq {
    self.types = types;
    self
  }

  pub fn multiple(mut self, multiple: Option<bool>) -> ZSelectSourcesReq {
    self.multiple = multiple;
    self
  }

  pub fn cursor_mode(mut self, cursor_mode: Option<u32>) -> ZSelectSourcesReq {
    self.cursor_mode = cursor_mode;
    self
  }

  pub fn restore_token(mut self, restore_token: Option<&str>) -> ZSelectSourcesReq {
    self.restore_token = restore_token.map(|token| token.to_string());
    self
  }

  pub fn persist_mode(mut self, persist_mode: Option<u32>) -> ZSelectSourcesReq {
    self.persist_mode = persist_mode;
    self
  }
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZStartReq {
  handle_token: String,
}

impl ZStartReq {
  pub fn new(handle_token: &str) -> ZStartReq {
    ZStartReq { handle_token: handle_token.to_string() }
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZStartRes {
  pub(crate) streams: Vec<(u32, ZStartStream)>,
  pub(crate) restore_token: Option<String>,
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZStartStream {
  pub(crate) id: Option<String>,
  pub(crate) position: Option<(i32, i32)>,
  pub(crate) size: Option<(i32, i32)>,
  pub(crate) source_type: u32,
  pub(crate) mapping_id: Option<String>,
}