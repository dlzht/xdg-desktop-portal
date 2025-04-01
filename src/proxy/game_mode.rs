use crate::errors::Result;
use zbus::proxy;
use zvariant::Fd;

/// Portal for accessing GameMode
#[proxy(
  interface = "org.freedesktop.portal.GameMode",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZGameMode {
  #[zbus(property, name = "version")]
  fn version(&self) -> Result<u32>;

  #[zbus(property, name = "Active")]
  fn active(&self) -> Result<bool>;

  fn query_status(&self, pid: i32) -> Result<i32>;

  fn register_game(&self, pid: i32) -> Result<i32>;

  fn unregister_game(&self, pid: i32) -> Result<i32>;

  fn query_status_by_pid(&self, target: i32, requester: i32) -> Result<i32>;

  fn register_game_by_pid(&self, target: i32, requester: i32) -> Result<i32>;

  fn unregister_game_by_pid(&self, target: i32, requester: i32) -> Result<i32>;

  #[zbus(name = "QueryStatusByPIDFd<'_>")]
  fn query_status_by_pid_fd(&self, target: Fd<'_>, requester: Fd<'_>) -> Result<i32>;

  #[zbus(name = "RegisterGameByPIDFd<'_>")]
  fn register_game_by_pid_fd(&self, target: Fd<'_>, requester: Fd<'_>) -> Result<i32>;

  #[zbus(name = "UnregisterGameByPIDFd<'_>")]
  fn unregister_game_by_pid_fd(&self, target: Fd<'_>, requester: Fd<'_>) -> Result<i32>;
}
