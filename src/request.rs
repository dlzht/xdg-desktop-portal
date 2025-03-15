use crate::errors::{Error, Result};
use serde::Deserialize;
use std::fmt::Debug;
use zbus::{proxy, Connection, Proxy};
use zvariant::OwnedObjectPath;

/// https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.portal.Request.xml

pub struct Request {
  connection: Connection,
  proxy: ZRequestProxy<'static>,
}

impl Request {
  pub async fn new(handle_token: &str, connection: &Connection) -> Result<Self> {
    let connection = connection.clone();
    let name = connection
      .unique_name()
      .ok_or(Error::EmptyUniqueName)?
      .replace(":", "")
      .replace(".", "_");
    let path = format!("/org/freedesktop/portal/desktop/request/{name}/{handle_token}");
    let proxy = ZRequestProxy::builder(&connection)
      .path(OwnedObjectPath::try_from(path)?)?
      .build()
      .await?;
    Ok(Request { connection, proxy })
  }

  pub async fn responses(&self) -> Result<ResponseStream> {
    let stream = self.proxy.receive_response().await?;
    Ok(stream)
  }

  pub async fn close(&self) -> Result<()> {
    self.proxy.close().await
  }
}

#[proxy(
  interface = "org.freedesktop.portal.Request",
  default_service = "org.freedesktop.portal.Desktop"
)]
pub trait ZRequest {
  fn close(&self) -> Result<()>;

  #[zbus(signal)]
  fn response<R>(&self, response: u32, results: R) -> Result<()>;
}