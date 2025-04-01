use crate::errors::Result;
use crate::proxy::game_mode::ZGameModeProxy;
use std::os::fd::AsFd;
use zbus::Connection;

pub struct GameModePortal {
  proxy: ZGameModeProxy<'static>,
}

impl GameModePortal {
  pub async fn new(connection: Connection) -> Result<Self> {
    let proxy = ZGameModeProxy::new(&connection).await?;
    let portal = GameModePortal { proxy };
    Ok(portal)
  }

  pub async fn active(&self) -> Result<bool> {
    self.proxy.active().await
  }

  pub async fn query_status(&self, pid: i32) -> Result<i32> {
    self.proxy.query_status(pid).await
  }

  pub async fn register_game(&self, pid: i32) -> Result<i32> {
    self.proxy.register_game(pid).await
  }

  pub async fn query_status_by_pid(&self, target: i32, requester: i32) -> Result<i32> {
    self.proxy.query_status_by_pid(target, requester).await
  }

  pub async fn register_game_by_pid(&self, target: i32, requester: i32) -> Result<i32> {
    self.proxy.register_game_by_pid(target, requester).await
  }

  pub async fn unregister_game_by_pid(&self, target: i32, requester: i32) -> Result<i32> {
    self.proxy.unregister_game_by_pid(target, requester).await
  }

  pub async fn query_status_by_pid_fd<T: AsFd, U: AsFd>(
    &self,
    target: T,
    requester: U,
  ) -> Result<i32> {
    let target = target.as_fd().into();
    let requester = requester.as_fd().into();
    self.proxy.query_status_by_pid_fd(target, requester).await
  }

  pub async fn register_game_by_pid_fd<T: AsFd, U: AsFd>(
    &self,
    target: T,
    requester: T,
  ) -> Result<i32> {
    let target = target.as_fd().into();
    let requester = requester.as_fd().into();
    self.proxy.register_game_by_pid_fd(target, requester).await
  }

  pub async fn unregister_game_by_pid_fd<T: AsFd, U: AsFd>(
    &self,
    target: T,
    requester: U,
  ) -> Result<i32> {
    let target = target.as_fd().into();
    let requester = requester.as_fd().into();
    self
      .proxy
      .unregister_game_by_pid_fd(target, requester)
      .await
  }
}
