use zbus::proxy;
use zvariant::{Fd, OwnedObjectPath};
use crate::errors::Result;

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

  fn set_wallpaper_uri(&self, parent_window: &str, uri: &str, options: ZSetWallpaperReq<'_>) -> Result<OwnedObjectPath>;

  fn set_wallpaper_file(&self, parent_window: &str, fd: Fd<'_>, options: ZSetWallpaperReq<'_>) -> Result<()>;
}

pub struct ZSetWallpaperReq<'a> {
  show_preview: Option<bool>,
  set_on: Option<&'a str>,
}