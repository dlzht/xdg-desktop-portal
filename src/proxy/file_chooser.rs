use crate::errors::Result;
use zbus::proxy;
use zvariant::{DeserializeDict, OwnedObjectPath, SerializeDict, Type};

/// File chooser portal
///
/// The FileChooser portal allows sandboxed applications to ask
/// the user for access to files outside the sandbox. The portal
/// backend will present the user with a file chooser dialog.
#[proxy(
  interface = "org.freedesktop.portal.FileChooser",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZFileChooser {
  fn open_file(
    &self,
    parent_window: &str,
    title: &str,
    options: &ZOpenFileReq<'_>,
  ) -> Result<OwnedObjectPath>;
  fn save_file(
    &self,
    parent_window: &str,
    title: &str,
    options: &ZSaveFileReq<'_>,
  ) -> Result<OwnedObjectPath>;
  fn save_files(
    &self,
    parent_window: &str,
    title: &str,
    options: &ZSaveFilesReq<'_>,
  ) -> Result<OwnedObjectPath>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZOpenFileReq<'a> {
  handle_token: &'a str,
  accept_label: Option<&'a str>,
  modal: Option<bool>,
  multiple: Option<bool>,
  directory: Option<bool>,
  filters: Option<&'a Vec<ZFileFilterReq<'a>>>,
  current_filter: Option<ZFileFilterReq<'a>>,
  current_folder: Option<&'a [u8]>,
  // choices: &'a str,
}

impl<'a> ZOpenFileReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZOpenFileReq {
      handle_token,
      accept_label: None,
      modal: None,
      multiple: None,
      directory: None,
      filters: None,
      current_filter: None,
      current_folder: None,
      // choices: "",
    }
  }

  pub fn accept_label(mut self, accept_label: Option<&'a str>) -> Self {
    self.accept_label = accept_label;
    self
  }

  pub fn modal(mut self, modal: Option<bool>) -> Self {
    self.modal = modal;
    self
  }

  pub fn multiple(mut self, multiple: Option<bool>) -> Self {
    self.multiple = multiple;
    self
  }

  pub fn directory(mut self, directory: Option<bool>) -> Self {
    self.directory = directory;
    self
  }

  pub fn filters(mut self, filters: Option<&'a Vec<ZFileFilterReq<'a>>>) -> Self {
    self.filters = filters;
    self
  }

  pub fn current_filters(mut self, filters: Option<ZFileFilterReq<'a>>) -> Self {
    self.current_filter = filters;
    self
  }

  pub fn current_folder(mut self, folder: Option<&'a [u8]>) -> Self {
    self.current_folder = folder;
    self
  }
}

pub type ZFileFilterReq<'a> = (&'a str, &'a Vec<(u32, String)>);

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZOpenFileRes {
  pub uris: Vec<String>,
  pub current_filter: Option<ZFileFilterRes>,
  // choices: Vec<String>,
}

pub type ZFileFilterRes = (String, Vec<(u32, String)>);

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZSaveFileReq<'a> {
  handle_token: &'a str,
  accept_label: Option<&'a str>,
  modal: Option<bool>,
  filters: Option<&'a Vec<ZFileFilterReq<'a>>>,
  current_filter: Option<ZFileFilterReq<'a>>,
  // choices: Option<&'a str>,
  current_name: Option<&'a str>,
  current_folder: Option<&'a [u8]>,
  current_file: Option<&'a [u8]>,
}

impl<'a> ZSaveFileReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZSaveFileReq {
      handle_token,
      accept_label: None,
      modal: None,
      filters: None,
      current_filter: None,
      // choices: "",
      current_name: None,
      current_folder: None,
      current_file: None,
    }
  }

  pub fn accept_label(mut self, accept_label: Option<&'a str>) -> Self {
    self.accept_label = accept_label;
    self
  }

  pub fn modal(mut self, modal: Option<bool>) -> Self {
    self.modal = modal;
    self
  }

  pub fn filters(mut self, filters: Option<&'a Vec<ZFileFilterReq<'a>>>) -> Self {
    self.filters = filters;
    self
  }

  pub fn current_filters(mut self, filters: Option<ZFileFilterReq<'a>>) -> Self {
    self.current_filter = filters;
    self
  }

  pub fn current_name(mut self, current_name: Option<&'a str>) -> Self {
    self.current_name = current_name;
    self
  }

  pub fn current_folder(mut self, current_folder: Option<&'a [u8]>) -> Self {
    self.current_folder = current_folder;
    self
  }

  pub fn current_file(mut self, current_file: Option<&'a [u8]>) -> Self {
    self.current_file = current_file;
    self
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZSaveFileRes {
  pub uris: Vec<String>,
  pub current_filter: Option<ZFileFilterRes>,
  // choices: Vec<String>,
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZSaveFilesReq<'a> {
  handle_token: &'a str,
  accept_label: Option<&'a str>,
  modal: Option<bool>,
  // choices: Option<&'a str>,
  files: Option<&'a Vec<&'a [u8]>>,
  current_folder: Option<&'a [u8]>,
}

impl<'a> ZSaveFilesReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZSaveFilesReq {
      handle_token,
      accept_label: None,
      modal: None,
      files: None,
      current_folder: None,
    }
  }

  pub fn accept_label(mut self, accept_label: Option<&'a str>) -> Self {
    self.accept_label = accept_label;
    self
  }

  pub fn modal(mut self, modal: Option<bool>) -> Self {
    self.modal = modal;
    self
  }

  pub fn files(mut self, files: Option<&'a Vec<&'a [u8]>>) -> Self {
    self.files = files;
    self
  }

  pub fn current_folder(mut self, current_folder: Option<&'a [u8]>) -> Self {
    self.current_folder = current_folder;
    self
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZSaveFilesRes {
  pub uris: Vec<String>,
  // pub choices: Vec<(String, String)>,
}
