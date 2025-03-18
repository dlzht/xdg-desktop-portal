use std::collections::HashMap;
use zbus::proxy;
use zvariant::{OwnedFd, OwnedObjectPath, SerializeDict, Type, Value};
use crate::errors::Result;

#[proxy(
  interface = "org.freedesktop.portal.Camera",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZCamera {

  fn access_camera(&self, options: &ZAccessCameraReq<'_>) -> Result<OwnedObjectPath>;

  fn open_pipe_wire_remote(&self, options: &HashMap<String, Value<'_>>) -> Result<OwnedFd>;

  #[zbus(property, name = "IsCameraPresent")]
  fn is_camera_present(&self) -> Result<bool>;

  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZAccessCameraReq<'a> {
  handle_token: &'a str,
}

impl<'a> ZAccessCameraReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZAccessCameraReq { handle_token }
  }
}