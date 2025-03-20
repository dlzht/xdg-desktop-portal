use xdg_desktop_portal::location::GetLocationsReq;
use xdg_desktop_portal::portal::Portal;
use zbus::export::ordered_stream::OrderedStreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let location_portal = portal.location().await.unwrap();
  let req = GetLocationsReq::new();
  let mut locations = location_portal.locations(req).await.unwrap();
  while let Some(location) = locations.next().await {
    println!("{:?}", location);
  }
}
