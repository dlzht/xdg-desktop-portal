use zbus::proxy;
use zvariant::{DeserializeDict, SerializeDict, Type};
use crate::errors::Result;

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct GetUserInformationReq {
  handle_token: String,
  reason: String,
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct GetUserInformationRes {
  id: String,
  name: String,
  image: String,
}

#[proxy(
  interface = "org.freedesktop.portal.Account",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/account"
)]
pub trait ZAccount {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  fn get_user_information(
    &self,
    window: &str,
    options: &GetUserInformationReq,
  ) -> Result<GetUserInformationRes>;
}
