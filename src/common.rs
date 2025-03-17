use bitflags::bitflags;

bitflags! {
  #[derive(Debug)]
  pub struct CursorMode: u32 {
    const Hidden = 1;
    const Embedded = 2;
    const Metadata = 4;
  }
}

bitflags! {
  #[derive(Debug)]
  pub struct SourceType: u32 {
    const Monitor = 1;
    const Window = 2;
    const Virtual = 4;
  }
}

bitflags! {
  #[derive(Debug)]
  pub struct PersistMode: u32 {
    const DoNotPersist = 0;
    const AsApplication = 1;
    const UntilRevoked = 2;
  }
}
