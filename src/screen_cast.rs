use crate::errors::Result;
use crate::request::ZRequestProxy;
use enumflags2::bitflags;
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::{proxy, Connection};
use zbus::export::ordered_stream::OrderedStreamExt;
use zvariant::{
  DeserializeDict, DynamicType, ObjectPath, OwnedFd, OwnedObjectPath, SerializeDict, Type,
};

/// https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.portal.ScreenCast.xml

pub struct ScreenCast {
  handle_token: String,
  session_token: String,
  connection: Connection,
  multiple: Option<bool>,
  source_type: Option<SourceType>,
  cursor_mode: Option<CursorMode>,
  persist_mode: Option<PersistMode>,
  restore_token: Option<String>,
}

impl ScreenCast {
  pub fn new(handle_token: &str, session_token: &str, connection: Connection) -> Self {
    ScreenCast {
      handle_token: handle_token.to_string(),
      session_token: session_token.to_string(),
      connection,
      multiple: None,
      source_type: None,
      cursor_mode: None,
      persist_mode: None,
      restore_token: None,
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

  pub async fn run(&mut self) -> Result<()> {
    Ok(())
  }
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
struct CreateSessionReq {
  handle_token: String,
  session_handle_token: String,
}

impl CreateSessionReq {
  fn new(handle_token: &str, session_handle_token: &str) -> CreateSessionReq {
    CreateSessionReq {
      handle_token: handle_token.to_string(),
      session_handle_token: session_handle_token.to_string(),
    }
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
struct CreateSessionRes {
  session_handle: String,
}

impl CreateSessionRes {
  fn session_handle(&self) -> Result<OwnedObjectPath> {
    Ok(OwnedObjectPath::try_from(self.session_handle.as_str())?)
  }
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
struct SelectSourcesReq {
  handle_token: String,
  types: Option<SourceType>,
  multiple: Option<bool>,
  cursor_mode: Option<CursorMode>,
  restore_token: Option<String>,
  persist_mode: Option<PersistMode>,
}

impl SelectSourcesReq {
  fn new(handle_token: &str) -> SelectSourcesReq {
    SelectSourcesReq {
      handle_token: handle_token.to_string(),
      types: None,
      multiple: None,
      cursor_mode: None,
      restore_token: None,
      persist_mode: None,
    }
  }
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
struct StartReq {
  handle_token: String,
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
struct StartRes {
  streams: Vec<StartStream>,
  restore_token: String,
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
struct StartStream {
  id: Option<String>,
  position: Option<(i32, i32)>,
  size: Option<(i32, i32)>,
  source_type: SourceType,
  mapping_id: Option<String>,
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CursorMode {
  Hidden = 1,
  Embedded = 2,
  Metadata = 4,
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SourceType {
  Monitor = 1,
  Window = 2,
  Virtual = 4,
}

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum PersistMode {
  DoNotPersist = 0,
  AsApplication = 1,
  UntilRevoked = 2,
}

#[proxy(
  interface = "org.freedesktop.portal.ScreenCast",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
trait ZScreenCast {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  #[zbus(property, name = "AvailableCursorModes")]
  fn available_cursor_modes(&self) -> Result<u32>;

  #[zbus(property, name = "AvailableSourceTypes")]
  fn available_source_types(&self) -> Result<u32>;

  fn create_session(&self, options: &CreateSessionReq) -> Result<OwnedObjectPath>;

  fn select_sources(
    &self,
    session_handle: ObjectPath<'_>,
    options: &SelectSourcesReq,
  ) -> Result<OwnedObjectPath>;

  fn start(
    &self,
    session_handle: ObjectPath<'_>,
    parent_window: &str,
    options: &SelectSourcesReq,
  ) -> Result<OwnedObjectPath>;

  fn open_pipe_wire_remote(self, options: &SelectSourcesReq) -> Result<OwnedFd>;
}