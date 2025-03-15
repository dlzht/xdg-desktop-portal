use crate::errors::Result;
use zbus::proxy;

#[proxy(
  interface = "org.freedesktop.portal.Request",
  default_service = "org.freedesktop.portal.Desktop"
)]
pub trait ZRequest {
  fn close(&self) -> Result<()>;

  #[zbus(signal)]
  fn response<R>(&self, response: u32, results: R) -> Result<()>;
}
