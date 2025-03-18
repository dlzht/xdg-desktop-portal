use crate::errors::Result;
use zbus::proxy;
use zvariant::{OwnedObjectPath, SerializeDict, Type};

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
  fn compose_email(
    &self,
    parent_window: &str,
    options: &ZComposeEmailReq<'_>,
  ) -> Result<OwnedObjectPath>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZComposeEmailReq<'a> {
  handle_token: Option<&'a str>,
  subject: Option<&'a str>,
  body: Option<&'a str>,
  address: Option<&'a str>,
  addresses: Option<&'a Vec<String>>,
  cc: Option<&'a Vec<String>>,
  bcc: Option<&'a Vec<String>>,
  attachment_fds: Option<&'a Vec<u32>>,
  activation_token: Option<&'a str>,
}

impl<'a> ZComposeEmailReq<'a> {
  pub fn new() -> Self {
    ZComposeEmailReq {
      handle_token: None,
      subject: None,
      body: None,
      address: None,
      addresses: None,
      cc: None,
      bcc: None,
      attachment_fds: None,
      activation_token: None,
    }
  }

  pub fn handle_token(mut self, handle_token: Option<&'a str>) -> Self {
    self.handle_token = handle_token;
    self
  }

  pub fn subject(mut self, subject: Option<&'a str>) -> Self {
    self.subject = subject;
    self
  }

  pub fn body(mut self, body: Option<&'a str>) -> Self {
    self.body = body;
    self
  }

  pub fn address(mut self, address: Option<&'a str>) -> Self {
    self.address = address;
    self
  }

  pub fn addresses(mut self, addresses: Option<&'a Vec<String>>) -> Self {
    self.addresses = addresses;
    self
  }

  pub fn cc(mut self, cc: Option<&'a Vec<String>>) -> Self {
    self.cc = cc;
    self
  }

  pub fn bcc(mut self, bcc: Option<&'a Vec<String>>) -> Self {
    self.bcc = bcc;
    self
  }

  pub fn attachment_fds(mut self, attachment_fds: Option<&'a Vec<u32>>) -> Self {
    self.attachment_fds = attachment_fds;
    self
  }

  pub fn activation_token(mut self, activation_token: Option<&'a str>) -> Self {
    self.activation_token = activation_token;
    self
  }
}
