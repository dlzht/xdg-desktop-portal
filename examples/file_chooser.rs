use std::path::PathBuf;
use xdg_portal::file_chooser::{FileFilterReq, OpenFileReq, SaveFileReq, SaveFilesReq};
use xdg_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  open_file().await;
}

async fn open_file() {
  let portal = Portal::new().await.unwrap();
  let mut file_chooser_portal = portal.file_chooser().await.unwrap();
  let filter1 = FileFilterReq::new("image-jpg".to_string(), vec!["*.jpg".to_string()]);
  let filter2 = FileFilterReq::new("image-png".to_string(), vec!["*.png".to_string()]);
  let filters = vec![filter1.clone(), filter2.clone()];
  let req = OpenFileReq::new()
    .title("This is open file title".to_string())
    .modal(true)
    .multiple(true)
    .filters(filters)
    .current_filter(filter2)
    .accept_label("This is accept label".to_string());
  let res = file_chooser_portal.open_file(req).await;
  println!("{:?}", res);
}

async fn save_file() {
  let portal = Portal::new().await.unwrap();
  let mut file_chooser_portal = portal.file_chooser().await.unwrap();
  let filter1 = FileFilterReq::new("image-jpg".to_string(), vec!["*.jpg".to_string()]);
  let filter2 = FileFilterReq::new("image-png".to_string(), vec!["*.png".to_string()]);
  let filters = vec![filter1.clone(), filter2.clone()];
  let req = SaveFileReq::new()
    .title("This is save file title".to_string())
    .modal(true)
    .filters(filters)
    .current_filter(filter2)
    .current_name("FILE_NAME".to_string())
    .accept_label("This is accept label".to_string());
  let res = file_chooser_portal.save_file(req).await;
  println!("{:?}", res);
}

async fn save_files() {
  let portal = Portal::new().await.unwrap();
  let mut file_chooser_portal = portal.file_chooser().await.unwrap();
  let path_buf1 = PathBuf::from("image-jpg1");
  let path_buf2 = PathBuf::from("image-jpg2");
  let req = SaveFilesReq::new()
    .title("This is save files title".to_string())
    .modal(true)
    .files(vec![path_buf1.clone(), path_buf2.clone()])
    .accept_label("This is accept label".to_string());
  let res = file_chooser_portal.save_files(req).await;
  println!("{:?}", res);
}
