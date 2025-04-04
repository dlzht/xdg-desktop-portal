use xdg_portal::portal::Portal;
use zbus::export::ordered_stream::OrderedStreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let network_monitor_portal = portal.network_monitor().await.unwrap();
  let res = network_monitor_portal.get_available().await;
  println!("available: {:?}", res);

  let res = network_monitor_portal.get_metered().await;
  println!("metered: {:?}", res);

  let res = network_monitor_portal.get_connectivity().await;
  println!("connectivity: {:?}", res);

  let res = network_monitor_portal.get_connectivity().await;
  println!("connectivity: {:?}", res);

  let res = network_monitor_portal.can_reach("github.com", 443).await;
  println!("can_reach: {:?}", res);

  let mut changed = network_monitor_portal.change().await.unwrap();
  while let Some(_signal) = changed.next().await {
    println!("receive changed signal");
  }
}
