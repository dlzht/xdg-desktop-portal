use crate::errors::Result;
use zbus::proxy;

/// Power Profile monitoring portal
///
/// The Power Profile Monitor interface provides information about the
/// user-selected system-wide power profile, to sandboxed applications.
#[proxy(
  interface = "org.freedesktop.portal.PowerProfileMonitor",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZPowerProfileMonitor {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  #[zbus(property, name = "power-saver-enabled")]
  fn power_saver_enabled(&self) -> Result<bool>;
}
