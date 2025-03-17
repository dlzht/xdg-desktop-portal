use zbus::Connection;
use zbus::export::ordered_stream::OrderedStreamExt;
use crate::proxy::memory_monitor::{LowMemoryWaringStream, ZMemoryMonitorProxy};
use crate::errors::{Result, Error};

/// Portal for memory monitoring
pub struct MemoryMonitorPortal {
  signals: LowMemoryWaringStream,
}

impl MemoryMonitorPortal {
  /// Create MemoryMonitorPortal instance
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(connection: Connection) -> Result<MemoryMonitorPortal> {
    let proxy = ZMemoryMonitorProxy::new(&connection).await?;
    let signals = proxy.receive_low_memory_waring().await?;
    let portal = MemoryMonitorPortal { signals };
    Ok(portal)
  }

  /// Returned u8 representing the level of low memory warning.
  ///
  /// Signal emitted when a particular low memory situation happens with 0 being the lowest
  /// level of memory availability warning, and 255 being the highest. Those levels are defined
  /// and documented in `Low Memory Monitor's documentation
  ///
  /// **50**: Memory on the device is low, processes should free up unneeded resources so they can be used elsewhere.
  ///
  /// **100**: The device has even less free memory, so processes should try harder to free up unneeded resources. If your process does not need to stay running, it is a good time for it to quit.
  ///
  /// **255**: The system will start terminating processes to reclaim memory, including background processes.
  pub async fn low_memory_warning(&mut self) -> Result<u8> {
    let signal = self.signals.next().await
      .ok_or(Error::SignalStreamClosed)?;
    let level = signal.args()?.level;
    Ok(level)
  }
}