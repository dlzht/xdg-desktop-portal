use xdg_portal::email::ComposeEmailReq;
use xdg_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let email_portal = portal.email().await.unwrap();
  let req = ComposeEmailReq::new()
    .subject("Email Subject")
    .body("Hello")
    .addresses(vec!["example@github.com".to_string()]);
  email_portal.compose_email(req).await.unwrap();
}
