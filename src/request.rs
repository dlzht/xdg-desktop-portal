use crate::errors::{Error, Result};
use crate::proxy::request::{ResponseStream, ZRequestProxy};
use zbus::Connection;
use zvariant::OwnedObjectPath;

/// shared request interface
pub struct RequestPortal {
  proxy: ZRequestProxy<'static>,
}

impl RequestPortal {
  /// create RequestPortal instance
  ///
  /// `handle_token`: string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(handle_token: &str, connection: Connection) -> Result<Self> {
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
    Ok(RequestPortal { proxy })
  }

  pub async fn responses(&self) -> Result<ResponseStream> {
    let stream = self.proxy.receive_response().await?;
    Ok(stream)
  }

  pub async fn close(&self) -> Result<()> {
    self.proxy.close().await
  }
}
