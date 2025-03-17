use zbus::proxy;
use zvariant::{DeserializeDict, OwnedObjectPath, SerializeDict, Type};
use crate::errors::Result;

/// Portal for obtaining information about the user
///
/// This simple interface lets sandboxed applications query basic
///  information about the user, like their name and avatar photo.
#[proxy(
  interface = "org.freedesktop.portal.Account",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZAccount {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  #[zbus(name = "GetUserInformation")]
  fn get_user_information(
    &self,
    window: &str,
    options: &ZGetUserInfoReq<'_>,
  ) -> Result<OwnedObjectPath>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZGetUserInfoReq<'a> {
  handle_token: &'a str,
  reason: Option<&'a str>,
}

impl<'a> ZGetUserInfoReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZGetUserInfoReq {
      handle_token,
      reason: None,
    }
  }

  pub fn reason(mut self, reason: Option<&'a str>) -> Self {
    self.reason = reason;
    self
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZGetUserInfoRes {
  pub id: String,
  pub name: String,
  pub image: String,
}


