use crate::errors::Result;
use zbus::proxy;
use zvariant::{DeserializeDict, ObjectPath, OwnedObjectPath, SerializeDict, Type};

/// Portal for obtaining information about the location
///
/// This simple interface lets sandboxed applications query basic
/// information about the location.
#[proxy(
  interface = "org.freedesktop.portal.Location",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZLocation {
  fn create_session(&self, options: &ZCreateSessionReq<'_>) -> Result<OwnedObjectPath>;

  fn start(
    &self,
    session_handle: ObjectPath<'_>,
    parent_window: &str,
    options: &ZLocationStartReq<'_>,
  ) -> Result<OwnedObjectPath>;

  #[zbus(signal)]
  fn location_updated(
    &self,
    session_handle: ObjectPath<'_>,
    location: ZLocationUpdatedRes,
  ) -> Result<()>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZCreateSessionReq<'a> {
  session_handle_token: &'a str,
  distance_threshold: Option<u32>,
  accuracy: Option<u32>,
}

impl<'a> ZCreateSessionReq<'a> {
  pub fn new(session_handle_token: &'a str) -> Self {
    ZCreateSessionReq {
      session_handle_token,
      distance_threshold: None,
      accuracy: None,
    }
  }

  pub fn distance_threshold(mut self, distance_threshold: Option<u32>) -> Self {
    self.distance_threshold = distance_threshold;
    self
  }

  pub fn accuracy(mut self, accuracy: Option<u32>) -> Self {
    self.accuracy = accuracy;
    self
  }
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZLocationStartReq<'a> {
  handle_token: &'a str,
}

impl<'a> ZLocationStartReq<'a> {
  pub fn new(handle_token: &'a str) -> Self {
    ZLocationStartReq { handle_token }
  }
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZLocationUpdatedRes {
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub altitude: Option<f64>,
  pub accuracy: Option<f64>,
  pub speed: Option<f64>,
  pub heading: Option<f64>,
  pub timestamp: Option<(u64, u64)>,
}
