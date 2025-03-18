use crate::errors::Result;
use crate::proxy::email::{ZComposeEmailReq, ZEmailProxy};
use zbus::Connection;

/// portal for sending email
pub struct EmailPortal {
  handle_token: String,
  proxy: ZEmailProxy<'static>,
}

impl EmailPortal {
  /// create EmailPortal instance
  ///
  /// `handle_token`: string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(handle_token: &str, connection: Connection) -> Result<EmailPortal> {
    let proxy = ZEmailProxy::new(&connection).await?;
    let portal = EmailPortal {
      handle_token: handle_token.to_string(),
      proxy,
    };
    Ok(portal)
  }

  /// presents a window that lets the user compose an email.
  pub async fn compose_email(&self, req: ComposeEmailReq) -> Result<()> {
    let req = ZComposeEmailReq::new()
      .subject(req.subject.as_deref())
      .body(req.body.as_deref())
      .addresses(req.addresses.as_ref())
      .cc(req.cc.as_ref())
      .bcc(req.bcc.as_ref())
      .attachment_fds(req.attach.as_ref())
      .activation_token(req.activation_token.as_deref());
    self.proxy.compose_email("", &req).await?;
    Ok(())
  }
}

#[derive(Debug)]
pub struct ComposeEmailReq {
  pub(crate) subject: Option<String>,
  pub(crate) body: Option<String>,
  pub(crate) attach: Option<Vec<u32>>,
  pub(crate) addresses: Option<Vec<String>>,
  pub(crate) cc: Option<Vec<String>>,
  pub(crate) bcc: Option<Vec<String>>,
  pub(crate) activation_token: Option<String>,
}

impl ComposeEmailReq {
  pub fn new() -> Self {
    ComposeEmailReq {
      subject: None,
      body: None,
      attach: None,
      addresses: None,
      cc: None,
      bcc: None,
      activation_token: None,
    }
  }

  pub fn subject(mut self, subject: &str) -> Self {
    self.subject = Some(subject.to_string());
    self
  }

  pub fn body(mut self, body: &str) -> Self {
    self.body = Some(body.to_string());
    self
  }

  pub fn addresses(mut self, addresses: Vec<String>) -> Self {
    self.addresses = Some(addresses);
    self
  }

  pub fn attach(mut self, attach: Vec<u32>) -> Self {
    self.attach = Some(attach);
    self
  }

  pub fn cc(mut self, cc: Vec<String>) -> Self {
    self.cc = Some(cc);
    self
  }

  pub fn bcc(mut self, bcc: Vec<String>) -> Self {
    self.bcc = Some(bcc);
    self
  }

  pub fn activation_token(mut self, activation_token: &str) -> Self {
    self.activation_token = Some(activation_token.to_string());
    self
  }
}
