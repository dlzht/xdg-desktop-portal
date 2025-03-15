use zbus::Connection;
use crate::proxy::account::ZAccountProxy;

pub struct Account {
  connection: Connection,
  proxy: ZAccountProxy<'static>,
}

impl Account {}

