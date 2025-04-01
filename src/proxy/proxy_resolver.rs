use crate::errors::Result;
use zbus::proxy;

/// Portal for proxy information
///
/// The ProxyResolver interface provides network proxy information to sandboxed
/// applications. It is not a portal in the strict sense, since it does not involve
/// user interaction. Applications are expected to use this interface indirectly,
/// via a library API such as the GLib GProxyResolver interface.
#[proxy(
  interface = "org.freedesktop.portal.ProxyResolver",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZProxyResolver {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  fn lookup(&self, uri: &str) -> Result<Vec<String>>;
}
