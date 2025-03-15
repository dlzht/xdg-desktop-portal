use crate::errors::Result;
use zbus::proxy;

/// Shared request interface
///
/// The Request interface is shared by all portal interfaces. When a
/// portal method is called, the reply includes a handle (i.e. object path)
/// for a Request object, which will stay alive for the duration of the
/// user interaction related to the method call.
///
/// The portal indicates that a portal request interaction is over by
/// emitting the #org.freedesktop.portal.Request::Response signal on the
/// Request object.
///
/// The application can abort the interaction calling
/// org.freedesktop.portal.Request.Close() on the Request object.
#[proxy(
  interface = "org.freedesktop.portal.Request",
  default_service = "org.freedesktop.portal.Desktop"
)]
pub trait ZRequest {
  fn close(&self) -> Result<()>;

  #[zbus(signal)]
  fn response<R>(&self, response: u32, results: R) -> Result<()>;
}
