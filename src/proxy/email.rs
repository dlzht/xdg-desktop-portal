use zbus::proxy;
use zvariant::{OwnedObjectPath, SerializeDict, Type};
use crate::errors::Result;

#[derive(SerializeDict, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct ZComposeEmailReq {
  pub(crate) handle_token: Option<String>,
  pub(crate) subject: Option<String>,
  pub(crate) body: Option<String>,
  pub(crate) address: Option<String>,
  pub(crate) addresses: Option<Vec<String>>,
  pub(crate) cc: Option<Vec<String>>,
  pub(crate) bcc: Option<Vec<String>>,
  pub(crate) attachment_fds: Option<Vec<u32>>,
  pub(crate) activation_token: Option<String>,
}

/// Portal for sending email
///
/// This simple portal lets sandboxed applications request to send an email,
/// optionally providing an address, subject, body and attachments.
#[proxy(
  interface = "org.freedesktop.portal.Email",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZEmail {

  #[zbus(name = "ComposeEmail")]
  fn compose_email(&self, parent_window: &str, options: &ZComposeEmailReq) -> Result<OwnedObjectPath>;

}