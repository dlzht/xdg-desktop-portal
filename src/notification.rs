use crate::common::{NotificationCategory, NotificationDisplayHit, NotificationPriority};
use crate::errors::Result;
use crate::proxy::notification::{ZNotificationProxy, ZSendNotificationReq};
use std::ops::Deref;
use zbus::Connection;
use zvariant::Value;

/// portal for sending notifications
pub struct NotificationPortal {
  proxy: ZNotificationProxy<'static>,
}

impl NotificationPortal {
  /// create NotificationPortal instance
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(connection: Connection) -> Result<Self> {
    let proxy = ZNotificationProxy::new(&connection).await?;
    let portal = NotificationPortal { proxy };
    Ok(portal)
  }

  /// send notification
  pub async fn send_notification(&self, req: SendNotificationReq) -> Result<()> {
    let id = req.id.as_deref().unwrap_or("");
    let hits = req
      .display_hit
      .as_ref()
      .map(|h| h.iter().map(|h| h.into()).collect());
    let req = ZSendNotificationReq::new()
      .title(req.title.as_deref())
      .body(req.body.as_deref())
      .category(req.category.as_ref().map(|c| c.into()))
      .priority(req.priority.as_ref().map(|p| p.into()))
      .default_action(req.default_action.as_deref())
      .display_hit(hits.as_ref());
    self.proxy.send_notification(id, &req).await
  }

  /// remove notification
  pub async fn remove_notification(&self, req: RemoveNotificationReq) -> Result<()> {
    self.proxy.remove_notification(req.id.deref()).await
  }
}

/// request of [`NotificationPortal::send_notification`]
pub struct SendNotificationReq {
  pub(crate) id: Option<String>,
  pub(crate) title: Option<String>,
  pub(crate) body: Option<String>,
  pub(crate) priority: Option<NotificationPriority>,
  pub(crate) category: Option<NotificationCategory>,
  pub(crate) display_hit: Option<Vec<NotificationDisplayHit>>,
  pub(crate) default_action: Option<String>,
  pub(crate) default_action_target: Option<Value<'static>>, // icon
                                                            // sound
                                                            // button
}

impl SendNotificationReq {
  pub fn new() -> Self {
    SendNotificationReq {
      id: None,
      title: None,
      body: None,
      priority: None,
      category: None,
      display_hit: None,
      default_action: None,
      default_action_target: None,
    }
  }

  pub fn id(mut self, id: String) -> Self {
    self.id = Some(id);
    self
  }

  pub fn title(mut self, title: String) -> Self {
    self.title = Some(title);
    self
  }

  pub fn body(mut self, body: String) -> Self {
    self.body = Some(body);
    self
  }

  pub fn priority(mut self, priority: NotificationPriority) -> Self {
    self.priority = Some(priority);
    self
  }

  pub fn category(mut self, category: NotificationCategory) -> Self {
    self.category = Some(category);
    self
  }

  pub fn display_hit(mut self, display_hit: Vec<NotificationDisplayHit>) -> Self {
    self.display_hit = Some(display_hit);
    self
  }

  pub fn default_action(mut self, default_action: String) -> Self {
    self.default_action = Some(default_action);
    self
  }

  pub fn default_action_target(mut self, default_action_target: Value<'static>) -> Self {
    self.default_action_target = Some(default_action_target);
    self
  }
}

/// request of [`NotificationPortal::remove_notification`]
pub struct RemoveNotificationReq {
  pub(crate) id: String,
}

impl RemoveNotificationReq {
  pub fn new(id: String) -> Self {
    RemoveNotificationReq { id }
  }
}
