use crate::errors::Result;
use crate::proxy::email::{ZComposeEmailReq, ZEmailProxy};
use zbus::Connection;

/// portal for sending email
pub struct EmailPortal {
  connection: Connection,
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
      connection,
      handle_token: handle_token.to_string(),
      proxy,
    };
    Ok(portal)
  }

  /// presents a window that lets the user compose an email.
  pub async fn compose_email(&self, req: ComposeEmailReq) -> Result<()> {
    let req = self.trans_send_email_req(req);
    self.proxy.compose_email("", &req).await?;
    Ok(())
  }

  fn trans_send_email_req(&self, req: ComposeEmailReq) -> ZComposeEmailReq {
    let ComposeEmailReq {
      subject,
      body,
      attach,
      addresses,
      cc,
      bcc,
      activation_token
    } = req;
    ZComposeEmailReq {
      handle_token: Some(self.handle_token.to_string()),
      subject,
      body,
      addresses: if addresses.is_empty() { None } else { Some(addresses) },
      address: None,
      attachment_fds: if attach.is_empty() { None } else { Some(attach) },
      cc: if cc.is_empty() { None } else { Some(cc) },
      bcc: if bcc.is_empty() { None } else { Some(bcc) },
      activation_token
    }
  }
}

#[derive(Default, Debug)]
pub struct ComposeEmailReq {
  pub(crate) subject: Option<String>,
  pub(crate) body: Option<String>,
  pub(crate) attach: Vec<u32>,
  pub(crate) addresses: Vec<String>,
  pub(crate) cc: Vec<String>,
  pub(crate) bcc: Vec<String>,
  pub(crate) activation_token: Option<String>,
}

impl ComposeEmailReq {
  pub fn subject(mut self, subject: &str) -> Self {
    self.subject = Some(subject.to_string());
    self
  }

  pub fn body(mut self, body: &str) -> Self {
    self.body = Some(body.to_string());
    self
  }

  pub fn addresses(mut self, addresses: Vec<String>) -> Self {
    self.addresses = addresses;
    self
  }

  pub fn attach(mut self, attach: Vec<u32>) -> Self {
    self.attach = attach;
    self
  }

  pub fn cc(mut self, cc: Vec<String>) -> Self {
    self.cc = cc;
    self
  }

  pub fn bcc(mut self, bcc: Vec<String>) -> Self {
    self.bcc = bcc;
    self
  }

  pub fn activation_token(mut self, activation_token: &str) -> Self {
    self.activation_token = Some(activation_token.to_string());
    self
  }
}
