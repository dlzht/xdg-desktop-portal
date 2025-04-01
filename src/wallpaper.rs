use crate::common::WallpaperLocation;
use crate::errors::Result;
use crate::proxy::wallpaper::{ZSetWallpaperReq, ZWallpaperProxy};
use std::fmt::Debug;
use std::os::fd::AsFd;
use zbus::Connection;

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
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let uri = req.uri.as_str();
    let req = ZSetWallpaperReq::new()
      .show_preview(req.show_preview)
      .set_on(req.set_on.as_ref().map(|s| s.into()));
    let _ = self
      .proxy
      .set_wallpaper_uri(parent_window, uri, req)
      .await?;
    Ok(())
  }

  pub async fn set_wallpaper_file<T: AsFd + Debug>(
    &self,
    req: SetWallpaperFileReq<T>,
  ) -> Result<()> {
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let fd = req.fd.as_fd();
    let req = ZSetWallpaperReq::new()
      .show_preview(req.show_preview)
      .set_on(req.set_on.as_ref().map(|s| s.into()));
    let _ = self
      .proxy
      .set_wallpaper_file(parent_window, fd.into(), req)
      .await?;
    Ok(())
  }
}

#[derive(Debug)]
pub struct SetWallpaperUriReq {
  parent_window: Option<String>,
  show_preview: Option<bool>,
  set_on: Option<WallpaperLocation>,
  uri: String,
}

impl SetWallpaperUriReq {
  pub fn new(uri: String) -> Self {
    SetWallpaperUriReq {
      parent_window: None,
      show_preview: None,
      set_on: None,
      uri,
    }
  }

  pub fn parent_window(mut self, parent_window: String) -> Self {
    self.parent_window = Some(parent_window);
    self
  }

  pub fn show_preview(mut self, show_preview: bool) -> Self {
    self.show_preview = Some(show_preview);
    self
  }

  pub fn set_on(mut self, set_on: WallpaperLocation) -> Self {
    self.set_on = Some(set_on);
    self
  }
}

#[derive(Debug)]
pub struct SetWallpaperFileReq<T: AsFd + Debug> {
  parent_window: Option<String>,
  show_preview: Option<bool>,
  set_on: Option<WallpaperLocation>,
  fd: T,
}

impl<T: AsFd + Debug> SetWallpaperFileReq<T> {
  pub fn new(fd: T) -> Self {
    SetWallpaperFileReq {
      parent_window: None,
      show_preview: None,
      set_on: None,
      fd,
    }
  }

  pub fn parent_window(mut self, parent_window: String) -> Self {
    self.parent_window = Some(parent_window);
    self
  }

  pub fn show_preview(mut self, show_preview: bool) -> Self {
    self.show_preview = Some(show_preview);
    self
  }

  pub fn set_on(mut self, set_on: WallpaperLocation) -> Self {
    self.set_on = Some(set_on);
    self
  }
}
