use std::os::fd::AsFd;
use xdg_portal::common::WallpaperLocation;
use xdg_portal::portal::Portal;
use xdg_portal::wallpaper::{SetWallpaperFileReq, SetWallpaperUriReq, WallpaperPortal};

#[tokio::main(flavor = "current_thread")]
async fn main() {
  set_wallpaper_url().await;
}

async fn set_wallpaper_url() {
  let portal = Portal::new().await.unwrap();
  let wallpaper_portal = portal.wallpaper().await.unwrap();
  let req = SetWallpaperUriReq::new("example.png".to_string())
    .show_preview(true)
    .set_on(WallpaperLocation::Background);
  wallpaper_portal.set_wallpaper_uri(req).await.unwrap();
}

async fn set_wallpaper_file() {
  let portal = Portal::new().await.unwrap();
  let wallpaper_portal = portal.wallpaper().await.unwrap();
  let file = std::fs::File::open("example.png").unwrap();
  let req = SetWallpaperFileReq::new(file.as_fd())
    .show_preview(true)
    .set_on(WallpaperLocation::Background);
  wallpaper_portal.set_wallpaper_file(req).await.unwrap();
}
