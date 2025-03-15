use enumflags2::bitflags;
use serde_repr::{Deserialize_repr, Serialize_repr};
use zvariant::Type;

pub struct Version(u32);

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CursorMode {
  Hidden = 1,
  Embedded = 2,
  Metadata = 4,
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SourceType {
  Monitor = 1,
  Window = 2,
  Virtual = 4,
}

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum PersistMode {
  DoNotPersist = 0,
  AsApplication = 1,
  UntilRevoked = 2,
}