use std::os::fd::AsFd;
use zbus::Connection;
use crate::common::WallpaperLocation;
use crate::proxy::wallpaper::ZWallpaperProxy;
use crate::errors::Result;

/// Portal for setting the desktop's Wallpaper
pub struct WallpaperPortal {
  proxy: ZWallpaperProxy<'static>,
}

impl WallpaperPortal {
  pub async fn new(connection: Connection) -> Result<Self> {
    let proxy = ZWallpaperProxy::new(&connection).await?;
    let portal = WallpaperPortal { proxy };
    Ok(portal)
  }

  pub async fn set_wallpaper_uri(&self, req: SetWallpaperUriReq) -> Result<()> {
    unimplemented!()
  }

  pub async fn set_wallpaper_file(&self, req: SetWallpaperUriReq) -> Result<()> {
    unimplemented!()
  }
}

pub struct SetWallpaperUriReq {
  parent_window: Option<String>,
  show_preview: Option<bool>,
  set_on: Option<WallpaperLocation>,
  uri: String,
}

pub struct SetWallpaperFileReq<T: AsFd> {
  parent_window: Option<String>,
  show_preview: Option<bool>,
  set_on: Option<WallpaperLocation>,
  fd: T
}
