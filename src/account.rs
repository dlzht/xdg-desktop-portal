use crate::errors::{Error, Result};
use crate::proxy::account::{ZGetUserInfoReq, ZGetUserInfoRes, ZAccountProxy};
use crate::proxy::request::ResponseStream;
use crate::request::RequestPortal;
use zbus::export::ordered_stream::OrderedStreamExt;
use zbus::Connection;

/// portal for obtaining information about the user
pub struct AccountPortal {
  handle_token: String,
  proxy: ZAccountProxy<'static>,
  signals: ResponseStream,
}

impl AccountPortal {
  /// create AccountPortal instance
  ///
  /// `handle_token`: string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(handle_token: &str, connection: Connection) -> Result<AccountPortal> {
    let proxy = ZAccountProxy::new(&connection).await?;
    let responses = RequestPortal::new(handle_token, connection)
      .await?
      .responses()
      .await?;
    let portal = AccountPortal {
      handle_token: handle_token.to_string(),
      proxy,
      signals: responses,
    };
    Ok(portal)
  }

  /// get information about the user.
  pub async fn get_user_information(
    &mut self,
    req: GetUserInfoReq,
  ) -> Result<GetUserInfoRes> {
    let GetUserInfoReq {window, reason} = req;
    let req = ZGetUserInfoReq::new(self.handle_token.as_str())
      .reason(reason.as_deref());
    let _ = self.proxy.get_user_information(window.as_deref().unwrap_or(""), &req).await?;
    let signal = self
      .signals
      .next()
      .await
      .ok_or(Error::SignalStreamClosed)?
      .args::<ZGetUserInfoRes>()?
      .results;
    Ok(create_user_info_res(signal))
  }
}

/// request of [`AccountPortal::get_user_information`]
#[derive(Debug)]
pub struct GetUserInfoReq {
  pub(crate) window: Option<String>,
  pub(crate) reason: Option<String>,
}

impl GetUserInfoReq {

  /// create [``GetUserInfoReq] instance
  pub fn new() -> GetUserInfoReq {
    GetUserInfoReq {
      window: None,
      reason: None,
    }
  }

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

fn create_user_info_res(res: ZGetUserInfoRes) -> GetUserInfoRes {
  let ZGetUserInfoRes { id, name, image } = res;
  GetUserInfoRes { id, name, image }
}
