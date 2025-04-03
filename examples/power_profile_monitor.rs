use xdg_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let power_monitor_portal = portal.power_profile_monitor().await.unwrap();
  let res = power_monitor_portal.power_saver_enabled().await;
  println!("{:?}", res);
}
