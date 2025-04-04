use crate::errors::Result;
use zbus::proxy;
use zvariant::{DeserializeDict, Type};

/// Network monitoring portal
///
/// The NetworkMonitor interface provides network status information
/// to sandboxed applications
#[proxy(
  interface = "org.freedesktop.portal.NetworkMonitor",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZNetworkMonitor {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  #[zbus(signal)]
  fn changed(&self);

  fn get_available(&self) -> Result<bool>;

  fn get_metered(&self) -> Result<bool>;

  fn get_connectivity(&self) -> Result<u32>;

  fn get_status(&self) -> Result<ZGetStatusRes>;

  fn can_reach(&self, hostname: &str, port: u32) -> Result<bool>;
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZGetStatusRes {
  pub available: bool,
  pub metered: bool,
  pub connectivity: u32,
}
