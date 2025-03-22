use crate::account::AccountPortal;
use crate::camera::CameraPortal;
use crate::email::EmailPortal;
use crate::errors::Result;
use crate::file_chooser::FileChooserPortal;
use crate::location::LocationPortal;
use crate::memory_monitor::MemoryMonitorPortal;
use crate::notification::NotificationPortal;
use crate::screencast::ScreencastPortal;
use crate::screenshot::ScreenshotPortal;
use crate::trash::TrashPortal;
use std::cell::Cell;
use zbus::Connection;
use crate::proxy_resolver::ProxyResolverPortal;

pub struct Portal {
  connection: Connection,
  last_counter: Cell<usize>,
}

impl Portal {
  pub async fn new() -> Result<Portal> {
    let connection = Connection::session().await?;
    let portal = Portal {
      connection,
      last_counter: Cell::new(0),
    };
    Ok(portal)
  }

  pub async fn screencast(&self) -> Result<ScreencastPortal> {
    self.increase_counter();
    let token = self.last_counter.get().to_string();
    ScreencastPortal::new(token.as_str(), token.as_str(), self.connection.clone()).await
  }

  pub async fn account(&self) -> Result<AccountPortal> {
    self.increase_counter();
    let token = self.last_counter.get().to_string();
    AccountPortal::new(token.as_str(), self.connection.clone()).await
  }

  pub async fn memory_monitor(&self) -> Result<MemoryMonitorPortal> {
    MemoryMonitorPortal::new(self.connection.clone()).await
  }

  pub async fn email(&self) -> Result<EmailPortal> {
    EmailPortal::new(self.connection.clone()).await
  }

  pub async fn notification(&self) -> Result<NotificationPortal> {
    NotificationPortal::new(self.connection.clone()).await
  }

  pub async fn trash(&self) -> Result<TrashPortal> {
    TrashPortal::new(self.connection.clone()).await
  }

  pub async fn camera(&self) -> Result<CameraPortal> {
    self.increase_counter();
    let token = self.last_counter.get().to_string();
    CameraPortal::new(token, self.connection.clone()).await
  }

  pub async fn file_chooser(&self) -> Result<FileChooserPortal> {
    self.increase_counter();
    let token = self.last_counter.get().to_string();
    FileChooserPortal::new(token, self.connection.clone()).await
  }

  pub async fn location(&self) -> Result<LocationPortal> {
    self.increase_counter();
    let token = self.last_counter.get().to_string();
    LocationPortal::new(token, self.connection.clone()).await
  }

  pub async fn screenshot(&self) -> Result<ScreenshotPortal> {
    self.increase_counter();
    let token = self.last_counter.get().to_string();
    ScreenshotPortal::new(token, self.connection.clone()).await
  }

  pub async fn proxy_resolver(&self) -> Result<ProxyResolverPortal> {
    ProxyResolverPortal::new(self.connection.clone()).await
  }

  fn increase_counter(&self) {
    self.last_counter.update(|c| c.wrapping_add(1));
  }
}
