use crate::common::NetworkConnectivity;
use crate::errors::{Error, Result};
use crate::proxy::network_monitor::{ZGetStatusRes, ZNetworkMonitorProxy};
use zbus::Connection;
use zbus::export::ordered_stream::{OrderedStream, OrderedStreamExt};

pub struct NetworkMonitorPortal {
  proxy: ZNetworkMonitorProxy<'static>,
}

impl NetworkMonitorPortal {
  /// create PowerProfileMonitorPortal instance
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(connection: Connection) -> Result<Self> {
    let proxy = ZNetworkMonitorProxy::new(&connection).await?;
    let portal = NetworkMonitorPortal { proxy };
    Ok(portal)
  }

  /// emitted when the network configuration changes
  pub async fn change(&self) -> Result<impl OrderedStream<Data = ()>> {
    let stream = self.proxy.receive_changed().await?;
    Ok(stream.map(|_a| ()))
  }

  ///  whether the network is available
  pub async fn get_available(&self) -> Result<bool> {
    self.proxy.get_available().await
  }

  ///  whether the network is metered
  pub async fn get_metered(&self) -> Result<bool> {
    self.proxy.get_metered().await
  }

  /// returns more detailed information about the host's network connectivity
  pub async fn get_connectivity(&self) -> Result<NetworkConnectivity> {
    let res = self.proxy.get_connectivity().await?;
    u32_to_network_connectivity(res)
  }

  /// returns the three values all at once
  pub async fn get_status(&self) -> Result<GetNetworkStatusRes> {
    let res = self.proxy.get_status().await?;
    create_network_status_res(res)
  }

  /// returns whether the given hostname is believed to be reachable
  pub async fn can_reach(&self, hostname: &str, port: u32) -> Result<bool> {
    self.proxy.can_reach(hostname, port).await
  }
}

fn u32_to_network_connectivity(network_connectivity: u32) -> Result<NetworkConnectivity> {
  match network_connectivity {
    1 => Ok(NetworkConnectivity::LocalOnly),
    2 => Ok(NetworkConnectivity::Limited),
    3 => Ok(NetworkConnectivity::CaptivePortal),
    4 => Ok(NetworkConnectivity::FullNetwork),
    _ => Err(Error::UnknownConnectivity(network_connectivity)),
  }
}

/// response for [`NetworkMonitorPortal::get_status`]
#[derive(Debug)]
pub struct GetNetworkStatusRes {
  pub available: bool,
  pub metered: bool,
  pub connectivity: NetworkConnectivity,
}

fn create_network_status_res(res: ZGetStatusRes) -> Result<GetNetworkStatusRes> {
  let connectivity = u32_to_network_connectivity(res.connectivity)?;
  let status = GetNetworkStatusRes {
    available: res.available,
    metered: res.metered,
    connectivity,
  };
  Ok(status)
}
