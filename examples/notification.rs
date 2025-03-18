use std::time::Duration;
use xdg_desktop_portal::common::NotificationPriority;
use xdg_desktop_portal::notification::{RemoveNotificationReq, SendNotificationReq};
use xdg_desktop_portal::portal::Portal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let portal = Portal::new().await.unwrap();
  let notification_portal = portal.notification().await.unwrap();
  let send_req = SendNotificationReq::new()
    .id("1".to_string())
    .title("This is title".to_string())
    .body("this is a high priority notification".to_string())
    .priority(NotificationPriority::High);
  let _ = notification_portal.send_notification(send_req).await;
  tokio::time::sleep(Duration::from_millis(1000)).await;
  let remove_req = RemoveNotificationReq::new("1".to_string());
  let _ = notification_portal.remove_notification(remove_req).await;
}