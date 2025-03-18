use crate::errors::Result;
use std::collections::HashMap;
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

  fn create_session(&self, options: &ZCreateSessionReq<'_>) -> Result<OwnedObjectPath>;

  fn select_sources(
    &self,
    session_handle: ObjectPath<'_>,
    options: &ZSelectSourcesReq<'_>,
  ) -> Result<OwnedObjectPath>;

  fn start(
    &self,
    session_handle: ObjectPath<'_>,
    parent_window: &str,
    options: &ZStartReq<'_>,
  ) -> Result<OwnedObjectPath>;

  fn open_pipe_wire_remote(
    &self,
    session_handle: ObjectPath<'_>,
    options: HashMap<String, Value<'_>>,
  ) -> Result<OwnedFd>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZCreateSessionReq<'a> {
  handle_token: &'a str,
  session_handle_token: &'a str,
}

impl<'a> ZCreateSessionReq<'a> {
  pub fn new(handle_token: &'a str, session_handle_token: &'a str) -> ZCreateSessionReq<'a> {
    ZCreateSessionReq {
      handle_token,
      session_handle_token,
    }
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZCreateSessionRes {
  pub session_handle: String,
}

impl ZCreateSessionRes {
  pub fn session_handle(&self) -> Result<OwnedObjectPath> {
    Ok(OwnedObjectPath::try_from(self.session_handle.as_str())?)
  }
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZSelectSourcesReq<'a> {
  handle_token: &'a str,
  types: Option<u32>,
  multiple: Option<bool>,
  cursor_mode: Option<u32>,
  restore_token: Option<&'a str>,
  persist_mode: Option<u32>,
}

impl<'a> ZSelectSourcesReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZSelectSourcesReq {
      handle_token,
      types: None,
      multiple: None,
      cursor_mode: None,
      restore_token: None,
      persist_mode: None,
    }
  }

  pub fn types(mut self, types: Option<u32>) -> Self {
    self.types = types;
    self
  }

  pub fn multiple(mut self, multiple: Option<bool>) -> Self {
    self.multiple = multiple;
    self
  }

  pub fn cursor_mode(mut self, cursor_mode: Option<u32>) -> Self {
    self.cursor_mode = cursor_mode;
    self
  }

  pub fn restore_token(mut self, restore_token: Option<&'a str>) -> Self {
    self.restore_token = restore_token;
    self
  }

  pub fn persist_mode(mut self, persist_mode: Option<u32>) -> Self {
    self.persist_mode = persist_mode;
    self
  }
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZStartReq<'a> {
  handle_token: &'a str,
}

impl<'a> ZStartReq<'a> {
  pub fn new(handle_token: &'a str) -> ZStartReq<'a> {
    ZStartReq { handle_token }
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZStartRes {
  pub streams: Vec<(u32, ZStartStream)>,
  pub restore_token: Option<String>,
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZStartStream {
  pub id: Option<String>,
  pub position: Option<(i32, i32)>,
  pub size: Option<(i32, i32)>,
  pub source_type: u32,
  pub mapping_id: Option<String>,
}
