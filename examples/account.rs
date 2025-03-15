use xdg_desktop_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let mut account_portal = portal.account().await.unwrap();
  println!("{:?}", account_portal.get_user_information(None, Some("I want to get user info")).await);
}