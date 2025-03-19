use std::os::fd::AsFd;
use xdg_desktop_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let trash_portal = portal.trash().await.unwrap();
  let file = tokio::fs::File::open("some_file").await.unwrap();
  trash_portal.trash_file(file.as_fd()).await.unwrap();
}
