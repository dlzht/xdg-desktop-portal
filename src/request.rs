use crate::errors::{Error, Result};
use serde::Deserialize;
use std::fmt::Debug;
use zbus::{proxy, Connection};
use zvariant::OwnedObjectPath;

/// https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.portal.Request.xml

const PATH: &str = "/org/freedesktop/portal/desktop/request/{}/{}";

#[proxy(
  interface = "org.freedesktop.portal.Request",
  default_service = "org.freedesktop.portal.Desktop"
)]
pub trait ZRequest {
  fn close(&self) -> Result<()>;

  #[zbus(signal)]
  fn response<R>(&self, response: u32, results: R) -> Result<()>;
}

impl ZRequestProxy<'_> {
  pub(crate) async fn signal_stream(
    handle_token: &str,
    connection: &Connection,
  ) -> Result<ResponseStream> {
    let name = connection
      .unique_name()
      .ok_or(Error::EmptyUniqueName)?
      .replace(":", "")
      .replace(".", "_");
    let path = format!("/org/freedesktop/portal/desktop/request/{name}/{handle_token}");
    let result = ZRequestProxy::builder(connection)
      .path(OwnedObjectPath::try_from(path)?)?
      .build()
      .await?
      .receive_response()
      .await?;
    Ok(result)
  }
}
