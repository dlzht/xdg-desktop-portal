use bitflags::bitflags;

bitflags! {
  #[derive(Debug)]
  pub struct CursorMode: u32 {
    const Hidden = 1;
    const Embedded = 2;
    const Metadata = 4;
  }
}

bitflags! {
  #[derive(Debug)]
  pub struct SourceType: u32 {
    const Monitor = 1;
    const Window = 2;
    const Virtual = 4;
  }
}

bitflags! {
  #[derive(Debug)]
  pub struct PersistMode: u32 {
    const DoNotPersist = 0;
    const AsApplication = 1;
    const UntilRevoked = 2;
  }
}

#[derive(Debug)]
pub enum NotificationPriority {
  Low,
  Normal,
  High,
  Urgent,
}

impl From<&NotificationPriority> for &str {
  fn from(value: &NotificationPriority) -> Self {
    match value {
      NotificationPriority::Low => "low",
      NotificationPriority::Normal => "normal",
      NotificationPriority::High => "high",
      NotificationPriority::Urgent => "urgent",
    }
  }
}

#[derive(Debug)]
pub enum NotificationCategory {
  ImReceived,
  AlarmRinging,
  CallIncoming,
  CallOngoing,
  CallUnanswered,
  WeatherWarningExtreme,
  CellBroadcastDangerExtreme,
  CellBroadcastDangerServer,
  CellBroadcastAmberAlert,
  CellBroadcastTest,
  OsBatteryLow,
  BrowserWebNotification,
  Other(String),
}

impl<'a> From<&'a NotificationCategory> for &'a str {
  fn from(value: &'a NotificationCategory) -> Self {
    match value {
      NotificationCategory::ImReceived => "im.received",
      NotificationCategory::AlarmRinging => "alarm.ringing",
      NotificationCategory::CallIncoming => "call.incoming",
      NotificationCategory::CallOngoing => "call.ongoing",
      NotificationCategory::CallUnanswered => "call.unanswered",
      NotificationCategory::WeatherWarningExtreme => "weather.warning.extreme",
      NotificationCategory::CellBroadcastDangerExtreme => "cellbroadcast.danger.extreme",
      NotificationCategory::CellBroadcastDangerServer => "cellbroadcast.danger.severe",
      NotificationCategory::CellBroadcastAmberAlert => "cellbroadcast.amber-alert",
      NotificationCategory::CellBroadcastTest => "cellbroadcast.test",
      NotificationCategory::OsBatteryLow => "os.battery.low",
      NotificationCategory::BrowserWebNotification => "browser.web-notification",
      NotificationCategory::Other(other) => other.as_str(),
    }
  }
}

#[derive(Debug)]
pub enum NotificationDisplayHit {
  Transient,
  Tray,
  Persistent,
  HideOnLockscreen,
  HideContentOnLockscreen,
  ShowAsNew,
}

impl From<&NotificationDisplayHit> for &str {
  fn from(value: &NotificationDisplayHit) -> Self {
    match value {
      NotificationDisplayHit::Transient => "transient",
      NotificationDisplayHit::Tray => "tray",
      NotificationDisplayHit::Persistent => "persistent",
      NotificationDisplayHit::HideOnLockscreen => "hide-on-lockscreen",
      NotificationDisplayHit::HideContentOnLockscreen => "hide-content-on-lockscreen",
      NotificationDisplayHit::ShowAsNew => "show-as-new",
    }
  }
}
