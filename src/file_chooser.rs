use crate::errors::{Error, Result};
use crate::proxy::file_chooser::{
  ZFileChooserProxy, ZFileFilterReq, ZFileFilterRes, ZOpenFileReq, ZOpenFileRes, ZSaveFileReq,
  ZSaveFileRes, ZSaveFilesReq, ZSaveFilesRes,
};
use crate::proxy::request::ResponseStream;
use crate::request::RequestPortal;
use std::path::{Path, PathBuf};
use zbus::Connection;
use zbus::export::ordered_stream::OrderedStreamExt;

/// portal for choosing file
pub struct FileChooserPortal {
  handle_token: String,
  proxy: ZFileChooserProxy<'static>,
  signals: ResponseStream,
}

impl FileChooserPortal {
  pub async fn new(handle_token: String, connection: Connection) -> Result<Self> {
    let proxy = ZFileChooserProxy::new(&connection).await?;
    let signals = RequestPortal::new(handle_token.as_str(), connection)
      .await?
      .responses()
      .await?;
    let portal = FileChooserPortal {
      handle_token,
      proxy,
      signals,
    };
    Ok(portal)
  }

  pub async fn open_file(&mut self, req: OpenFileReq) -> Result<OpenFileRes> {
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let title = req.title.as_deref().unwrap_or("");
    let filters = req
      .filters
      .as_ref()
      .map(|filters| filters.iter().map(|f| f.into()).collect());
    let req = ZOpenFileReq::new(self.handle_token.as_str())
      .accept_label(req.accept_label.as_deref())
      .modal(req.modal)
      .multiple(req.multiple)
      .directory(req.directory)
      .filters(filters.as_ref())
      .current_filters(req.current_filter.as_ref().map(ZFileFilterReq::from))
      .current_folder(
        req
          .current_folder
          .as_ref()
          .map(|f| f.as_os_str().as_encoded_bytes()),
      );
    let _ = self.proxy.open_file(parent_window, title, &req).await?;
    let signal = self
      .signals
      .next()
      .await
      .ok_or(Error::SignalStreamClosed)?
      .args::<ZOpenFileRes>()?
      .results;
    Ok(OpenFileRes::from(signal))
  }

  pub async fn save_file(&mut self, req: SaveFileReq) -> Result<SaveFileRes> {
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let title = req.title.as_deref().unwrap_or("");
    let filters = req
      .filters
      .as_ref()
      .map(|filters| filters.iter().map(|f| f.into()).collect());
    let req = ZSaveFileReq::new(self.handle_token.as_str())
      .accept_label(req.accept_label.as_deref())
      .modal(req.modal)
      .filters(filters.as_ref())
      .current_filters(req.current_filter.as_ref().map(ZFileFilterReq::from))
      .current_folder(
        req
          .current_folder
          .as_ref()
          .map(|f| f.as_os_str().as_encoded_bytes()),
      )
      .current_file(
        req
          .current_file
          .as_ref()
          .map(|f| f.as_os_str().as_encoded_bytes()),
      )
      .current_name(req.current_name.as_deref());
    let _ = self.proxy.save_file(parent_window, title, &req).await?;
    let signal = self
      .signals
      .next()
      .await
      .ok_or(Error::SignalStreamClosed)?
      .args::<ZSaveFileRes>()?
      .results;
    Ok(SaveFileRes::from(signal))
  }

  pub async fn save_files(&mut self, req: SaveFilesReq) -> Result<SaveFilesRes> {
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let title = req.title.as_deref().unwrap_or("");
    let req = ZSaveFilesReq::new(self.handle_token.as_str())
      .accept_label(req.accept_label.as_deref())
      .modal(req.modal)
      .current_folder(
        req
          .current_folder
          .as_ref()
          .map(|f| f.as_os_str().as_encoded_bytes()),
      );
    let _ = self.proxy.save_files(parent_window, title, &req).await?;
    let signal = self
      .signals
      .next()
      .await
      .ok_or(Error::SignalStreamClosed)?
      .args::<ZSaveFilesRes>()?
      .results;
    Ok(SaveFilesRes::from(signal))
  }
}

/// request of [`FileChooserPortal::open_file`]
#[derive(Debug)]
pub struct OpenFileReq {
  pub(crate) parent_window: Option<String>,
  pub(crate) title: Option<String>,
  pub(crate) accept_label: Option<String>,
  pub(crate) modal: Option<bool>,
  pub(crate) multiple: Option<bool>,
  pub(crate) directory: Option<bool>,
  pub(crate) filters: Option<Vec<FileFilterReq>>,
  pub(crate) current_filter: Option<FileFilterReq>,
  pub(crate) current_folder: Option<PathBuf>,
}

impl OpenFileReq {
  pub fn new() -> Self {
    OpenFileReq {
      parent_window: None,
      title: None,
      accept_label: None,
      modal: None,
      multiple: None,
      directory: None,
      filters: None,
      current_filter: None,
      current_folder: None,
    }
  }

  pub fn parent_window(mut self, parent_window: String) -> Self {
    self.parent_window = Some(parent_window);
    self
  }

  pub fn title(mut self, title: String) -> Self {
    self.title = Some(title);
    self
  }

  pub fn accept_label(mut self, accept_label: String) -> Self {
    self.accept_label = Some(accept_label);
    self
  }

  pub fn modal(mut self, modal: bool) -> Self {
    self.modal = Some(modal);
    self
  }

  pub fn multiple(mut self, multiple: bool) -> Self {
    self.multiple = Some(multiple);
    self
  }

  pub fn directory(mut self, directory: bool) -> Self {
    self.directory = Some(directory);
    self
  }

  pub fn filters(mut self, filters: Vec<FileFilterReq>) -> Self {
    self.filters = Some(filters);
    self
  }

  pub fn current_filter(mut self, current_filter: FileFilterReq) -> Self {
    self.current_filter = Some(current_filter);
    self
  }

  pub fn current_folder(mut self, current_folder: PathBuf) -> Self {
    self.current_folder = Some(current_folder);
    self
  }
}

/// response of [`FileChooserPortal::open_file`]
#[derive(Debug)]
pub struct OpenFileRes {
  uris: Vec<String>,
  current_filter: Option<FileFilterRes>,
}

impl From<ZOpenFileRes> for OpenFileRes {
  fn from(value: ZOpenFileRes) -> Self {
    let ZOpenFileRes {
      uris,
      current_filter,
    } = value;
    let current_filter = current_filter.map(|f| FileFilterRes::from(f));
    OpenFileRes {
      uris,
      current_filter,
    }
  }
}

/// request of [`FileChooserPortal::save_file`]
#[derive(Debug)]
pub struct SaveFileReq {
  pub(crate) parent_window: Option<String>,
  pub(crate) title: Option<String>,
  pub(crate) accept_label: Option<String>,
  pub(crate) modal: Option<bool>,
  pub(crate) filters: Option<Vec<FileFilterReq>>,
  pub(crate) current_filter: Option<FileFilterReq>,
  pub(crate) current_name: Option<String>,
  pub(crate) current_folder: Option<PathBuf>,
  pub(crate) current_file: Option<PathBuf>,
}

impl SaveFileReq {
  pub fn new() -> Self {
    SaveFileReq {
      parent_window: None,
      title: None,
      accept_label: None,
      modal: None,
      filters: None,
      current_filter: None,
      current_name: None,
      current_folder: None,
      current_file: None,
    }
  }

  pub fn parent_window(mut self, parent_window: String) -> Self {
    self.parent_window = Some(parent_window);
    self
  }

  pub fn title(mut self, title: String) -> Self {
    self.title = Some(title);
    self
  }

  pub fn accept_label(mut self, accept_label: String) -> Self {
    self.accept_label = Some(accept_label);
    self
  }

  pub fn modal(mut self, modal: bool) -> Self {
    self.modal = Some(modal);
    self
  }

  pub fn filters(mut self, filters: Vec<FileFilterReq>) -> Self {
    self.filters = Some(filters);
    self
  }

  pub fn current_filter(mut self, current_filter: FileFilterReq) -> Self {
    self.current_filter = Some(current_filter);
    self
  }

  pub fn current_name(mut self, current_name: String) -> Self {
    self.current_name = Some(current_name);
    self
  }

  pub fn current_folder(mut self, current_folder: PathBuf) -> Self {
    self.current_folder = Some(current_folder);
    self
  }

  pub fn current_file(mut self, current_file: PathBuf) -> Self {
    self.current_file = Some(current_file);
    self
  }
}

/// response of [`FileChooserPortal::save_file`]
#[derive(Debug)]
pub struct SaveFileRes {
  uris: Vec<String>,
  current_filter: Option<FileFilterRes>,
}

impl From<ZSaveFileRes> for SaveFileRes {
  fn from(value: ZSaveFileRes) -> Self {
    let ZSaveFileRes {
      uris,
      current_filter,
    } = value;
    let current_filter = current_filter.map(|f| FileFilterRes::from(f));
    SaveFileRes {
      uris,
      current_filter,
    }
  }
}

/// request of [`FileChooserPortal::save_files`]
#[derive(Debug)]
pub struct SaveFilesReq {
  pub(crate) parent_window: Option<String>,
  pub(crate) title: Option<String>,
  pub(crate) accept_label: Option<String>,
  pub(crate) modal: Option<bool>,
  pub(crate) files: Option<Vec<PathBuf>>,
  pub(crate) current_folder: Option<PathBuf>,
}

impl SaveFilesReq {
  pub fn new() -> Self {
    SaveFilesReq {
      parent_window: None,
      title: None,
      accept_label: None,
      modal: None,
      files: None,
      current_folder: None,
    }
  }

  pub fn parent_window(mut self, parent_window: String) -> Self {
    self.parent_window = Some(parent_window);
    self
  }

  pub fn title(mut self, title: String) -> Self {
    self.title = Some(title);
    self
  }

  pub fn accept_label(mut self, accept_label: String) -> Self {
    self.accept_label = Some(accept_label);
    self
  }

  pub fn modal(mut self, modal: bool) -> Self {
    self.modal = Some(modal);
    self
  }

  pub fn files(mut self, files: Vec<PathBuf>) -> Self {
    self.files = Some(files);
    self
  }

  pub fn current_folder(mut self, current_folder: PathBuf) -> Self {
    self.current_folder = Some(current_folder);
    self
  }
}

/// response of [`FileChooserPortal::save_files`]
#[derive(Debug)]
pub struct SaveFilesRes {
  uris: Vec<String>,
}

impl From<ZSaveFilesRes> for SaveFilesRes {
  fn from(value: ZSaveFilesRes) -> Self {
    let ZSaveFilesRes { uris } = value;
    SaveFilesRes { uris }
  }
}

#[derive(Debug, Clone)]
pub struct FileFilterReq {
  name: String,
  matches: Vec<(u32, String)>,
}

impl FileFilterReq {
  pub fn new(name: String, matches: Vec<String>) -> Self {
    let matches = matches
      .into_iter()
      .enumerate()
      .map(|(i, m)| (i as u32, m))
      .collect();
    FileFilterReq { name, matches }
  }
}

impl<'a> From<&'a FileFilterReq> for ZFileFilterReq<'a> {
  fn from(value: &'a FileFilterReq) -> Self {
    (&value.name, &value.matches)
  }
}

#[derive(Debug)]
pub struct FileFilterRes {
  name: String,
  matches: Vec<String>,
}

impl From<ZFileFilterRes> for FileFilterRes {
  fn from(value: ZFileFilterRes) -> Self {
    let (name, matches) = value;
    let mut matches = matches
      .into_iter()
      .map(|(index, matches)| matches)
      .collect();
    FileFilterRes { name, matches }
  }
}
