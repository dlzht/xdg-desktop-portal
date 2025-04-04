use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  ZBus(zbus::Error),
  ZVariant(zvariant::Error),
  EmptyUniqueName,
  SignalStreamClosed,
  UnknownConnectivity(u32),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::ZBus(err) => write!(f, "ZBus error: {}", err),
      Error::ZVariant(err) => write!(f, "ZVariant error: {}", err),
      Error::EmptyUniqueName => write!(f, "The unique name is empty"),
      Error::SignalStreamClosed => write!(f, "The signal stream has been closed"),
      Error::UnknownConnectivity(connectivity) => {
        write!(f, "The connectivity is unknown: {}", connectivity)
      }
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Error::ZBus(err) => Some(err),
      Error::ZVariant(err) => Some(err),
      _ => None,
    }
  }
}

impl From<zbus::Error> for Error {
  fn from(value: zbus::Error) -> Error {
    Error::ZBus(value)
  }
}

impl From<zvariant::Error> for Error {
  fn from(value: zvariant::Error) -> Error {
    Error::ZVariant(value)
  }
}
