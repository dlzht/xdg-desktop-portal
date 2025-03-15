use crate::errors::{Error, Result};
use crate::proxy::account::{GetUserInformationReq, GetUserInformationRes, ZAccountProxy};
use crate::proxy::request::ResponseStream;
use crate::request::RequestPortal;
use zbus::export::ordered_stream::OrderedStreamExt;
use zbus::Connection;

/// Portal for obtaining information about the user
pub struct AccountPortal {
  connection: Connection,
  handle_token: String,
  proxy: ZAccountProxy<'static>,
  responses: ResponseStream,
}

impl AccountPortal {
  /// Create AccountPortal instance
  ///
  /// `handle_token`
  ///
  /// A string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the :ref:`org.freedesktop.portal.Request` documentation for
  /// more information about the @handle.
  ///
  /// `connection`
  ///
  /// A Z-Bus session connection
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
      responses,
    };
    Ok(portal)
  }

  /// Gets information about the user.
  ///
  /// `window`
  ///
  /// Identifier for the window
  ///
  /// `reason`
  ///
  /// A string that can be shown in the dialog to explain why the information is
  /// needed. This should be a complete sentence that explains what the application
  /// will do with the returned information, for example: "Allows your personal
  /// information to be included with recipes you share with your friends".
  pub async fn get_user_information(
    &mut self,
    window: Option<&str>,
    reason: Option<&str>,
  ) -> Result<AccountUserInformation> {
    let req = GetUserInformationReq::new(self.handle_token.as_str(), reason);
    let window = window.unwrap_or("");
    let _ = self.proxy.get_user_information(window, &req).await?;
    let res = self
      .responses
      .next()
      .await
      .ok_or(Error::RequestPathClosed)?
      .args::<GetUserInformationRes>()?
      .results;
    Ok(AccountUserInformation::from(res))
  }
}

/// response of [`AccountPortal::get_user_information`]
#[derive(Debug)]
pub struct AccountUserInformation {
  /// the user id
  pub id: String,

  /// the user's real name
  pub name: String,

  /// the URI of an image file for the user's avatar photo
  pub image: String,
}

impl From<GetUserInformationRes> for AccountUserInformation {
  fn from(value: GetUserInformationRes) -> Self {
    let GetUserInformationRes { id, name, image } = value;
    AccountUserInformation { id, name, image }
  }
}
