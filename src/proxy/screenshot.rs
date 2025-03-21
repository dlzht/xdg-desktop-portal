use crate::errors::Result;
use zbus::proxy;
use zvariant::{DeserializeDict, OwnedObjectPath, SerializeDict, Type};

/// Portal for taking screenshots
///
/// This simple portal lets sandboxed applications request a screenshot
#[proxy(
  interface = "org.freedesktop.portal.Screenshot",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZScreenshot {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  fn screenshot(
    &self,
    parent_window: &str,
    options: &ZScreenshotReq<'_>,
  ) -> Result<OwnedObjectPath>;

  fn pick_color(&self, parent_window: &str, options: &ZPickColorReq<'_>)
  -> Result<OwnedObjectPath>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZScreenshotReq<'a> {
  handle_token: &'a str,
  modal: Option<bool>,
  interactive: Option<bool>,
}

impl<'a> ZScreenshotReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZScreenshotReq {
      handle_token,
      modal: None,
      interactive: None,
    }
  }

  pub fn modal(mut self, modal: Option<bool>) -> Self {
    self.modal = modal;
    self
  }

  pub fn interactive(mut self, interactive: Option<bool>) -> Self {
    self.interactive = interactive;
    self
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZScreenshotRes {
  pub uri: String,
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZPickColorReq<'a> {
  handle_token: &'a str,
}

impl<'a> ZPickColorReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZPickColorReq { handle_token }
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZPickColorRes {
  pub color: (f64, f64, f64),
}
