use crate::errors::{Error, Result};
use crate::proxy::account::{ZGetUserInfoReq, ZGetUserInfoRes, ZAccountProxy};
use crate::proxy::request::ResponseStream;
use crate::request::RequestPortal;
use zbus::export::ordered_stream::OrderedStreamExt;
use zbus::Connection;

/// Portal for obtaining information about the user
pub struct AccountPortal {
  connection: Connection,
  handle_token: String,
  proxy: ZAccountProxy<'static>,
  signals: ResponseStream,
}

impl AccountPortal {
  /// Create AccountPortal instance
  ///
  /// `handle_token`: string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(handle_token: &str, connection: Connection) -> Result<AccountPortal> {
    let responses = RequestPortal::new(handle_token, connection.clone())
      .await?
      .responses()
      .await?;
    let proxy = ZAccountProxy::new(&connection).await?;
    let portal = AccountPortal {
      connection,
      handle_token: handle_token.to_string(),
      proxy,
      signals: responses,
    };
    Ok(portal)
  }

  /// Gets information about the user.
  ///
  /// `window`: identifier for the window
  ///
  /// `reason`: string that can be shown in the dialog to explain why the information is
  /// needed. This should be a complete sentence that explains what the application
  /// will do with the returned information, for example: "Allows your personal
  /// information to be included with recipes you share with your friends".
  pub async fn get_user_information(
    &mut self,
    req: GetUserInfoReq,
  ) -> Result<GetUserInfoRes> {
    let GetUserInfoReq {window, reason} = req;
    let req = ZGetUserInfoReq::new(self.handle_token.as_str(), reason.as_deref());
    let _ = self.proxy.get_user_information(window.as_deref().unwrap_or(""), &req).await?;
    let signal = self
      .signals
      .next()
      .await
      .ok_or(Error::SignalStreamClosed)?
      .args::<ZGetUserInfoRes>()?
      .results;
    Ok(GetUserInfoRes::from(signal))
  }
}

/// request of [`AccountPortal::get_user_information`]
#[derive(Default, Debug)]
pub struct GetUserInfoReq {
  pub(crate) window: Option<String>,
  pub(crate) reason: Option<String>,
}

impl GetUserInfoReq {
  /// set field window
  pub fn window(mut self, window: &str) -> Self {
    self.window = Some(window.to_string());
    self
  }

  /// set field reason
  pub fn reason(mut self, reason: &str) -> Self {
    self.reason = Some(reason.to_string());
    self
  }
}

/// response of [`AccountPortal::get_user_information`]
#[derive(Debug)]
pub struct GetUserInfoRes {
  /// the user id
  pub id: String,

  /// the user's real name
  pub name: String,

  /// the URI of an image file for the user's avatar photo
  pub image: String,
}

impl From<ZGetUserInfoRes> for GetUserInfoRes {
  fn from(value: ZGetUserInfoRes) -> Self {
    let ZGetUserInfoRes { id, name, image } = value;
    GetUserInfoRes { id, name, image }
  }
}
