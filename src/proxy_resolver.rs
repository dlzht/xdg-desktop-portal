use crate::errors::Result;
use crate::proxy::proxy_resolver::ZProxyResolverProxy;
use zbus::Connection;

pub struct ProxyResolverPortal {
  proxy: ZProxyResolverProxy<'static>,
}

impl ProxyResolverPortal {
  /// create ProxyResolverPortal instance
  ///
  /// `connection`: Z-Bus session connection
  pub async fn new(connection: Connection) -> Result<Self> {
    let proxy = ZProxyResolverProxy::new(&connection).await?;
    let portal = ProxyResolverPortal { proxy };
    Ok(portal)
  }

  /// Looks up which proxy to use to connect to uri
  pub async fn lookup(&self, uri: &str) -> Result<Vec<String>> {
    let res = self.proxy.lookup(uri).await?;
    Ok(res)
  }
}
