use crate::common::{CursorMode, PersistMode, SourceType};
use crate::errors::Result;
use zbus::Connection;

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
