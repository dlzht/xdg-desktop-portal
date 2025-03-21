use xdg_desktop_portal::portal::Portal;
use xdg_desktop_portal::screenshot::{PickColorReq, ScreenshotReq};

#[tokio::main(flavor = "current_thread")]
async fn main() {
  screenshot().await;
}

async fn screenshot() {
  let portal = Portal::new().await.unwrap();
  let mut screenshot_portal = portal.screenshot().await.unwrap();
  let req = ScreenshotReq::new().interactive(true).modal(false);
  let res = screenshot_portal.screenshot(req).await;
  println!("{:?}", res);
}

async fn pick_color() {
  let portal = Portal::new().await.unwrap();
  let mut screenshot_portal = portal.screenshot().await.unwrap();
  let req = PickColorReq::new();
  let res = screenshot_portal.pick_color(req).await;
  println!("{:?}", res);
}
