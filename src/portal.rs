use std::cell::Cell;
use zbus::Connection;
use crate::screen_cast::ScreenCast;

pub struct Portal {
  connection: Connection,
  last_counter: Cell<usize>,
}

impl Portal {
  pub fn screen_cast(&self) -> ScreenCast {
    let token = self.last_counter.get().to_string();
    ScreenCast::new(token.as_str(), token.as_str(), self.connection.clone())
  }

  fn increase_counter(&self) {
    self.last_counter.update(|c| c.wrapping_add(1));
  }
}