use crate::errors::Result;
use zbus::proxy;
use zvariant::Fd;

/// Portal for trashing files
///
/// This simple interface lets sandboxed applications send files to
/// the trashcan.
#[proxy(
  interface = "org.freedesktop.portal.Trash",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZTrash {

  fn trash_file(&self, fd: Fd<'_>) -> Result<u32>;

}