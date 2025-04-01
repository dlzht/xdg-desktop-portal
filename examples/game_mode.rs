use xdg_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let game_mode_portal = portal.game_mode().await.unwrap();
  let res1 = game_mode_portal.active().await;
  println!("res1: {:?}", res1);
}
