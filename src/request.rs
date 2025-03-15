use crate::errors::{Error, Result};
use crate::proxy::request::{ResponseStream, ZRequestProxy};
use std::fmt::Debug;
use zbus::Connection;
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
