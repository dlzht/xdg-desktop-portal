use xdg_desktop_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let mut memory_monitor_portal = portal.memory_monitor().await.unwrap();
  tokio::spawn(async move {
    if let Ok(level) = memory_monitor_portal.low_memory_warning().await {

    }
  });
}