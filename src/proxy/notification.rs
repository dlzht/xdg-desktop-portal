use std::collections::HashMap;
use zbus::proxy;
use zvariant::{SerializeDict, Type, Value};
use crate::errors::Result;

///  Portal for sending notifications
///
///  This simple interface lets sandboxed applications send and withdraw
///  notifications. It is not possible for the application to learn
///  if the notification was actually presented to the user. Not a
///  portal in the strict sense, since there is no user interaction.
#[proxy(
  interface = "org.freedesktop.portal.Notification",
  default_service = "org.freedesktop.portal.Desktop",
  default_path = "/org/freedesktop/portal/desktop"
)]
pub trait ZNotification {
  #[zbus(name = "AddNotification")]
  fn send_notification(&self, id: &str, options: &ZSendNotificationReq<'_>) -> Result<()>;

  fn remove_notification(&self, id: &str) -> Result<()>;

  #[zbus(property, name = "SupportedOptions")]
  fn supported_options(&self) -> Result<HashMap<String, Value<'_>>>;

  fn action_invoked(&self, id: String, action: String, parameter: Vec<Value<'_>>) -> Result<()>;
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ZSendNotificationReq<'a> {
  title: Option<&'a str>,
  body: Option<&'a str>,
  markup_body: Option<&'a str>,
  priority: Option<&'a str>,
  default_action: Option<&'a str>,
  default_action_target: Option<bool>,
  display_hit: Option<&'a Vec<&'a str>>,
  category: Option<&'a str>,
  // button: Option<bool>,
  // icon: Option<bool>,
  // sound: Option<bool>,
}

impl<'a> ZSendNotificationReq<'a> {
  pub fn new() -> Self {
    ZSendNotificationReq {
      title: None,
      body: None,
      markup_body: None,
      priority: None,
      default_action: None,
      default_action_target: None,
      display_hit: None,
      category: None,
      // icon: None,
      // sound: None,
      // button: None,
    }
  }

  pub fn title(mut self, title: Option<&'a str>) -> Self {
    self.title = title;
    self
  }

  pub fn body(mut self, body: Option<&'a str>) -> Self {
    self.body = body;
    self
  }

  pub fn markup_body(mut self, markup_body: Option<&'a str>) -> Self {
    self.markup_body = markup_body;
    self
  }

  pub fn priority(mut self, priority: Option<&'a str>) -> Self {
    self.priority = priority;
    self
  }

  pub fn default_action(mut self, default_action: Option<&'a str>) -> Self {
    self.default_action = default_action;
    self
  }

  pub fn default_action_target(mut self, default_action_target: Option<bool>) -> Self {
    self.default_action_target = default_action_target;
    self
  }


  pub fn display_hit(mut self, display_hit: Option<&'a Vec<&str>>) -> Self {
    self.display_hit = display_hit;
    self
  }

  pub fn category(mut self, category: Option<&'a str>) -> Self {
    self.category = category;
    self
  }

  // pub fn button(mut self, button: Option<bool>) -> Self {
  //   self.button = button;
  //   self
  // }
  //
  // pub fn icon(mut self, icon: Option<bool>) -> Self {
  //   self.icon = icon;
  //   self
  // }
  //
  // pub fn sound(mut self, sound: Option<bool>) -> Self {
  //   self.sound = sound;
  //   self
  // }
}

pub struct ZNotificationButton<'a> {
  label: &'a str,
  action: &'a str,
  purpose: &'a str,
  // target
}

pub enum ZNotificationIcon {

}