use zbus::proxy;
use crate::errors::Result;

/// Memory monitoring portal
///
/// The Memory Monitor interface provides information about low system
/// memory to sandboxed applications. It is not a portal in the strict
/// sense, since it does not involve user interaction. Applications are
/// expected to use this interface indirectly, via a library API
/// such as the GLib GMemoryMonitor interface.
#[proxy(
  interface = "org.freedesktop.portal.MemoryMonitor",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZMemoryMonitor {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  #[zbus(signal)]
  fn low_memory_waring(&self, level: u8) -> Result<()>;

}