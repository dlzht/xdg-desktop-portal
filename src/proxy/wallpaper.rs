use crate::errors::Result;
use zbus::proxy;
use zvariant::{Fd, OwnedObjectPath, SerializeDict, Type};

/// Portal for setting the desktop's Wallpaper
///
/// This simple interface lets sandboxed applications set the user's
/// desktop background picture.
#[proxy(
  interface = "org.freedesktop.portal.Wallpaper",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZWallpaper {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  fn set_wallpaper_uri(
    &self,
    parent_window: &str,
    uri: &str,
    options: ZSetWallpaperReq<'_>,
  ) -> Result<OwnedObjectPath>;

  fn set_wallpaper_file(
    &self,
    parent_window: &str,
    fd: Fd<'_>,
    options: ZSetWallpaperReq<'_>,
  ) -> Result<()>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZSetWallpaperReq<'a> {
  show_preview: Option<bool>,
  set_on: Option<&'a str>,
}

impl<'a> ZSetWallpaperReq<'a> {
  pub fn new() -> Self {
    ZSetWallpaperReq {
      show_preview: None,
      set_on: None,
    }
  }

  pub fn show_preview(mut self, show_preview: Option<bool>) -> Self {
    self.show_preview = show_preview;
    self
  }

  pub fn set_on(mut self, set_on: Option<&'a str>) -> Self {
    self.set_on = set_on;
    self
  }
}
