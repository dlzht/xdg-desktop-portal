use xdg_desktop_portal::common::SourceType;
use xdg_desktop_portal::portal::Portal;
use xdg_desktop_portal::screencast::ScreencastReq;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let mut screencast_portal = portal.screencast().await.unwrap();
  let screencast_req = ScreencastReq::new()
    .source_type(SourceType::Window | SourceType::Monitor);
  let res = screencast_portal.screencast(screencast_req).await;
  println!("screencast_portal returned: {:?}", res);
}