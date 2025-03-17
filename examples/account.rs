use xdg_desktop_portal::account::GetUserInfoReq;
use xdg_desktop_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let mut account_portal = portal.account().await.unwrap();
  let req = GetUserInfoReq::new()
    .reason("I want to get user info");
  println!("{:?}", account_portal.get_user_information(req).await);
}