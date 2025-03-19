use crate::errors::Result;
use crate::proxy::camera::{ZAccessCameraReq, ZCameraProxy};
use crate::proxy::request::{ResponseStream, ZRequestProxy};
use crate::request::RequestPortal;
use std::collections::HashMap;
use zbus::Connection;

/// portal for obtaining information about the user
pub struct CameraPortal {
  handle_token: String,
  proxy: ZCameraProxy<'static>,
  signals: ResponseStream,
}

impl CameraPortal {
  /// create CameraPortal instance
  ///
  /// `handle_token`: string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(handle_token: String, connection: Connection) -> Result<Self> {
    let proxy = ZCameraProxy::new(&connection).await?;
    let signals = RequestPortal::new(handle_token.as_str(), connection)
      .await?
      .responses()
      .await?;
    let portal = CameraPortal {
      handle_token,
      proxy,
      signals,
    };
    Ok(portal)
  }

  /// create camera session and open pipewire remote
  pub async fn open_camera(&self) -> Result<std::os::fd::OwnedFd> {
    let req = ZAccessCameraReq::new(self.handle_token.as_str());
    let _ = self.proxy.access_camera(&req).await?;
    let fd = self.proxy.open_pipe_wire_remote(&HashMap::new()).await?;
    Ok(fd.into())
  }

  /// whether there is any cameras available
  pub async fn is_camera_present(&self) -> Result<bool> {
    self.proxy.is_camera_present().await
  }
}
