use crate::common::{CursorMode, PersistMode, SourceType};
use crate::errors::Result;
use zbus::proxy;
use zvariant::{DeserializeDict, ObjectPath, OwnedFd, OwnedObjectPath, SerializeDict, Type};

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
