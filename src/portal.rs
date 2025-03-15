use crate::screencast::ScreencastPortal;
use std::cell::Cell;
use zbus::Connection;
use crate::account::AccountPortal;
use crate::errors::Result;

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

  pub fn screencast(&self) -> ScreencastPortal {
    self.increase_counter();
    let token = self.last_counter.get().to_string();
    ScreencastPortal::new(token.as_str(), token.as_str(), self.connection.clone())
  }

  pub async fn account(&self) -> Result<AccountPortal> {
    self.increase_counter();
    let token = self.last_counter.get().to_string();
    AccountPortal::new(token.as_str(), self.connection.clone()).await
  }

  fn increase_counter(&self) {
    self.last_counter.update(|c| c.wrapping_add(1));
  }
}
