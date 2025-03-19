#![feature(let_chains)]

use xdg_desktop_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let camera_portal = portal.camera().await.unwrap();
  if let Ok(res) = camera_portal.is_camera_present().await
    && res
  {
    let fd = camera_portal.open_camera().await.unwrap();
    println!("Opened camera: {:?}", fd);
  }
}
