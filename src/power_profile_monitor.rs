use crate::errors::Result;
use crate::proxy::power_profile_monitor::ZPowerProfileMonitorProxy;
use zbus::Connection;

pub struct PowerProfileMonitorPortal {
  proxy: ZPowerProfileMonitorProxy<'static>,
}

impl PowerProfileMonitorPortal {
  /// create PowerProfileMonitorPortal instance
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(connection: Connection) -> Result<Self> {
    let proxy = ZPowerProfileMonitorProxy::new(&connection).await?;
    let portal = PowerProfileMonitorPortal { proxy };
    Ok(portal)
  }

  /// whether “Power Saver” mode is enabled on the system
  pub async fn power_saver_enabled(&self) -> Result<bool> {
    self.proxy.power_saver_enabled().await
  }
}
