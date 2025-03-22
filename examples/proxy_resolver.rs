use xdg_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let proxy_resolver_portal = portal.proxy_resolver().await.unwrap();
  let res = proxy_resolver_portal.lookup("https://github.com")
    .await;
  println!("{:?}", res);
}