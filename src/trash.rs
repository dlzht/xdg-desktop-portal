use zbus::Connection;
use zvariant::Fd;
use crate::errors::Result;
use crate::proxy::trash::ZTrashProxy;

/// Portal for trashing files
pub struct TrashPortal {
  proxy: ZTrashProxy<'static>
}

impl TrashPortal {
  /// create TrashPortal instance
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(connection: Connection) -> Result<TrashPortal> {
    let proxy = ZTrashProxy::new(&connection).await?;
    let portal = TrashPortal { proxy };
    Ok(portal)
  }

  /// send file to trashcan
  pub async fn trash_file<'a, T: Into<Fd<'a>>>(&self, fd: T) -> Result<u32> {
    self.proxy.trash_file(fd.into()).await
  }
}