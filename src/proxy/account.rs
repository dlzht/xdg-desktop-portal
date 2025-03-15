use zbus::proxy;
use zvariant::{DeserializeDict, OwnedObjectPath, SerializeDict, Type};
use crate::errors::Result;

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct GetUserInformationReq {
  handle_token: String,
  reason: Option<String>,
}

impl GetUserInformationReq {
  pub fn new(handle_token: &str, reason: Option<&str>) -> GetUserInformationReq {
    GetUserInformationReq {
      handle_token: handle_token.to_string(),
      reason: reason.map(|x| x.to_string()),
    }
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct GetUserInformationRes {
  pub id: String,
  pub name: String,
  pub image: String,
}

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
    options: &GetUserInformationReq,
  ) -> Result<OwnedObjectPath>;
}
