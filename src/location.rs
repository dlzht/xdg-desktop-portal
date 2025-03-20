use crate::errors::{Error, Result};
use crate::proxy::location::{
  ZCreateSessionReq, ZLocationProxy, ZLocationStartReq, ZLocationUpdatedRes,
};
use zbus::Connection;
use zbus::export::ordered_stream::{OrderedStream, OrderedStreamExt};

/// Portal for obtaining information about the location
pub struct LocationPortal {
  handle_token: String,
  proxy: ZLocationProxy<'static>,
}

impl LocationPortal {
  /// create AccountPortal instance
  ///
  /// `handle_token`: string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(handle_token: String, connection: Connection) -> Result<Self> {
    let proxy = ZLocationProxy::new(&connection).await?;
    let portal = LocationPortal {
      handle_token: handle_token.to_string(),
      proxy,
    };
    Ok(portal)
  }

  /// get updated location information
  pub async fn locations(
    &self,
    req: GetLocationsReq,
  ) -> Result<impl OrderedStream<Data = Result<GetLocationsRes>>> {
    let parent_window = req.parent_window.as_deref().unwrap_or("");
    let create_session_req = ZCreateSessionReq::new(self.handle_token.as_str());
    let session = self.proxy.create_session(&create_session_req).await?;
    let start_req = ZLocationStartReq::new(self.handle_token.as_str());
    let _ = self
      .proxy
      .start(session.as_ref(), parent_window, &start_req)
      .await?;
    let stream = self
      .proxy
      .receive_location_updated()
      .await?
      .map(|l| l.args().map(|a| a.location))
      .map(|z| {
        z.map_err(|e| Error::ZBus(e))
          .map(|r| GetLocationsRes::from(r))
      });
    Ok(stream)
  }
}

#[derive(Debug)]
pub struct GetLocationsReq {
  pub(crate) parent_window: Option<String>,
}

impl GetLocationsReq {
  pub fn new() -> Self {
    GetLocationsReq {
      parent_window: None,
    }
  }

  pub fn parent_window(mut self, parent_window: String) -> Self {
    self.parent_window = Some(parent_window.into());
    self
  }
}

#[derive(Debug)]
pub struct GetLocationsRes {
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub altitude: Option<f64>,
  pub accuracy: Option<f64>,
  pub speed: Option<f64>,
  pub heading: Option<f64>,
  pub timestamp: Option<(u64, u64)>,
}

impl From<ZLocationUpdatedRes> for GetLocationsRes {
  fn from(value: ZLocationUpdatedRes) -> Self {
    let ZLocationUpdatedRes {
      latitude,
      longitude,
      altitude,
      accuracy,
      speed,
      heading,
      timestamp,
    } = value;
    GetLocationsRes {
      latitude,
      longitude,
      altitude,
      accuracy,
      speed,
      heading,
      timestamp,
    }
  }
}
