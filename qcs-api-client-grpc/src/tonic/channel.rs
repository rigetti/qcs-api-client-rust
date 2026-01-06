//! Utilities for creating and configuring gRPC channels.
//!
//! The [`ChannelBuilder`] is the primary entry point for configuring a gRPC channel.
use std::time::Duration;

use backoff::ExponentialBackoff;
use http::{uri::InvalidUri, Uri};
use hyper_proxy2::{Intercept, Proxy, ProxyConnector};
use hyper_socks2::{Auth, SocksConnector};
use hyper_util::client::legacy::connect::HttpConnector;
use tonic::{
    body::Body,
    client::GrpcService,
    transport::{Channel, ClientTlsConfig, Endpoint},
};
use tower::{Layer, ServiceBuilder};
use url::Url;

use qcs_api_client_common::{
    backoff::{self, default_backoff},
    configuration::{tokens::TokenRefresher, ClientConfiguration, LoadError, TokenError},
};

#[cfg(feature = "tracing")]
use qcs_api_client_common::tracing_configuration::TracingConfiguration;

#[cfg(feature = "tracing")]
use super::trace::{build_trace_layer, CustomTraceLayer, CustomTraceService};
use super::{Error, RefreshLayer, RefreshService, RetryLayer, RetryService};

/// Errors that may occur when configuring a channel connection
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ChannelError {
    /// Failed to parse URI.
    #[error("Failed to parse URI: {0}")]
    InvalidUri(#[from] InvalidUri),
    /// Failed to parse URL. Used to derive user/pass.
    #[error("Failed to parse URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    /// Unsupported proxy protocol.
    #[error("Protocol is missing or not supported: {0:?}")]
    UnsupportedProtocol(Option<String>),
    /// Proxy ssl verification failed
    #[error("HTTP proxy ssl verification failed: {0}")]
    SslFailure(#[from] std::io::Error),
    /// Proxy targets do not agree
    #[error("Cannot set separate https and http proxies if one of them is socks5")]
    Mismatch {
        /// The URI of the HTTPS proxy.
        https_proxy: Uri,
        /// The URI of the HTTP proxy.
        http_proxy: Uri,
    },
}

/// Defines a logic for turning some object into a [`GrpcService`].
pub trait IntoService<C: GrpcService<Body>> {
    /// The service type that will be returned.
    type Service: GrpcService<Body>;

    /// Convert the object into a service.
    fn into_service(self, channel: C) -> Self::Service;
}

impl<C> IntoService<C> for ()
where
    C: GrpcService<Body>,
{
    type Service = C;
    fn into_service(self, channel: C) -> Self::Service {
        channel
    }
}

/// Options for configuring QCS authentication.
#[derive(Clone, Debug)]
pub struct RefreshOptions<O, R>
where
    R: TokenRefresher + Clone + Send + Sync,
{
    layer: RefreshLayer<R>,
    other: O,
}

impl<T> From<T> for RefreshOptions<(), T>
where
    T: TokenRefresher + Clone + Send + Sync,
{
    fn from(refresher: T) -> Self {
        Self {
            layer: RefreshLayer::with_refresher(refresher),
            other: (),
        }
    }
}

impl<C, T, O> IntoService<C> for RefreshOptions<O, T>
where
    C: GrpcService<Body>,
    O: IntoService<C>,
    O::Service: GrpcService<Body>,
    RefreshService<O::Service, T>: GrpcService<Body>,
    T: TokenRefresher + Clone + Send + Sync + 'static,
{
    type Service = RefreshService<O::Service, T>;
    fn into_service(self, channel: C) -> Self::Service {
        let service = self.other.into_service(channel);
        self.layer.layer(service)
    }
}

/// Options for configuring retry logic.
#[derive(Clone, Debug)]
pub struct RetryOptions<O = ()> {
    layer: RetryLayer,
    other: O,
}

impl From<ExponentialBackoff> for RetryOptions<()> {
    fn from(backoff: ExponentialBackoff) -> Self {
        Self {
            layer: RetryLayer { backoff },
            other: (),
        }
    }
}

impl<C, O> IntoService<C> for RetryOptions<O>
where
    C: GrpcService<Body>,
    O: IntoService<C>,
    O::Service: GrpcService<Body>,
    RetryService<O::Service>: GrpcService<Body>,
{
    type Service = RetryService<O::Service>;
    fn into_service(self, channel: C) -> Self::Service {
        let service = self.other.into_service(channel);
        self.layer.layer(service)
    }
}

/// Builder for configuring a [`Channel`].
#[derive(Clone, Debug)]
pub struct ChannelBuilder<O = ()> {
    endpoint: Endpoint,
    #[cfg(feature = "tracing")]
    trace_layer: CustomTraceLayer,
    options: O,
}

impl From<Endpoint> for ChannelBuilder<()> {
    fn from(endpoint: Endpoint) -> Self {
        #[cfg(feature = "tracing")]
        {
            let base_url = endpoint.uri().to_string();
            Self {
                endpoint,
                trace_layer: build_trace_layer(base_url, None),
                options: (),
            }
        }

        #[cfg(not(feature = "tracing"))]
        return Self {
            endpoint,
            options: (),
        };
    }
}

impl ChannelBuilder<()> {
    /// Create a [`ChannelBuilder`] using the given [`Uri`]
    pub fn from_uri(uri: Uri) -> Self {
        #[cfg(feature = "tracing")]
        {
            let base_url = uri.to_string();
            Self {
                endpoint: get_endpoint(uri),
                trace_layer: build_trace_layer(base_url, None),
                options: (),
            }
        }

        #[cfg(not(feature = "tracing"))]
        return Self {
            endpoint: get_endpoint(uri),
            options: (),
        };
    }
}

#[cfg(feature = "tracing")]
type TargetService = CustomTraceService;
#[cfg(not(feature = "tracing"))]
type TargetService = Channel;

impl<O> ChannelBuilder<O>
where
    O: IntoService<TargetService>,
{
    /// Wrap the channel with a timeout.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.endpoint = self.endpoint.timeout(timeout);
        self
    }

    /// Wrap the channel with the given [`RefreshLayer`].
    pub fn with_refresh_layer<T>(
        self,
        layer: RefreshLayer<T>,
    ) -> ChannelBuilder<RefreshOptions<O, T>>
    where
        T: TokenRefresher + Clone + Send + Sync,
    {
        #[cfg(feature = "tracing")]
        return ChannelBuilder {
            endpoint: self.endpoint,
            trace_layer: self.trace_layer,
            options: RefreshOptions {
                layer,
                other: self.options,
            },
        };
        #[cfg(not(feature = "tracing"))]
        return ChannelBuilder {
            endpoint: self.endpoint,
            options: RefreshOptions {
                layer,
                other: self.options,
            },
        };
    }

    /// Wrap the channel with QCS authentication using the given [`TokenRefresher`].
    pub fn with_token_refresher<T>(self, refresher: T) -> ChannelBuilder<RefreshOptions<O, T>>
    where
        T: TokenRefresher + Clone + Send + Sync,
    {
        self.with_refresh_layer(RefreshLayer::with_refresher(refresher))
    }

    /// Wrap the channel with QCS authentication for the given [`ClientConfiguration`].
    pub fn with_qcs_config(
        self,
        config: ClientConfiguration,
    ) -> ChannelBuilder<RefreshOptions<O, ClientConfiguration>> {
        #[cfg(feature = "tracing")]
        {
            let base_url = self.endpoint.uri().to_string();
            let trace_layer = build_trace_layer(base_url, config.tracing_configuration());
            let mut builder = self.with_token_refresher(config);
            builder.trace_layer = trace_layer;
            builder
        }
        #[cfg(not(feature = "tracing"))]
        {
            self.with_token_refresher(config)
        }
    }

    /// Wrap the channel with QCS authentication for the given QCS profile.
    ///
    /// # Errors
    ///
    /// Returns a [`LoadError`] if the profile cannot be loaded.
    pub fn with_qcs_profile(
        self,
        profile: Option<String>,
    ) -> Result<ChannelBuilder<RefreshOptions<O, ClientConfiguration>>, LoadError> {
        let config = match profile {
            Some(profile) => ClientConfiguration::load_profile(profile)?,
            None => ClientConfiguration::load_default()?,
        };

        Ok(self.with_qcs_config(config))
    }

    /// Wrap the channel with the given [`RetryLayer`].
    pub fn with_retry_layer(self, layer: RetryLayer) -> ChannelBuilder<RetryOptions<O>> {
        #[cfg(feature = "tracing")]
        return ChannelBuilder {
            endpoint: self.endpoint,
            trace_layer: self.trace_layer,
            options: RetryOptions {
                layer,
                other: self.options,
            },
        };
        #[cfg(not(feature = "tracing"))]
        return ChannelBuilder {
            endpoint: self.endpoint,
            options: RetryOptions {
                layer,
                other: self.options,
            },
        };
    }

    /// Wrap the channel with the given [`ExponentialBackoff`] configuration.
    pub fn with_retry_backoff(
        self,
        backoff: ExponentialBackoff,
    ) -> ChannelBuilder<RetryOptions<O>> {
        self.with_retry_layer(RetryLayer { backoff })
    }

    /// Wrap the channel with the default retry logic. See [`default_backoff`].
    pub fn with_default_retry(self) -> ChannelBuilder<RetryOptions<O>> {
        self.with_retry_backoff(default_backoff())
    }

    /// Build the [`Channel`]
    ///
    /// # Errors
    ///
    /// Returns a [`ChannelError`] if the service cannot be built.
    #[allow(clippy::result_large_err)]
    pub fn build(self) -> Result<O::Service, ChannelError> {
        let channel = get_channel_with_endpoint(&self.endpoint)?;
        #[cfg(feature = "tracing")]
        {
            let traced_channel = self.trace_layer.layer(channel);
            Ok(self.options.into_service(traced_channel))
        }

        #[cfg(not(feature = "tracing"))]
        Ok(self.options.into_service(channel))
    }
}

/// Parse a string as a URI.
///
/// This serves as a helper to avoid consumers needing to create a new error just to include this.
///
/// # Errors
///
/// [`Error::InvalidUri`] if the string is an invalid URI.
#[allow(clippy::result_large_err)]
pub fn parse_uri(s: &str) -> Result<Uri, Error<TokenError>> {
    s.parse().map_err(Error::from)
}

/// Get an [`Endpoint`] for the given [`Uri`]
#[allow(clippy::missing_panics_doc)]
pub fn get_endpoint(uri: Uri) -> Endpoint {
    Channel::builder(uri)
        .user_agent(concat!(
            "QCS gRPC Client (Rust)/",
            env!("CARGO_PKG_VERSION")
        ))
        .expect("user agent string should be valid")
        .tls_config(ClientTlsConfig::new().with_enabled_roots())
        .expect("tls setup should succeed")
}

/// Get an [`Endpoint`] for the given [`Uri`] and timeout.
pub fn get_endpoint_with_timeout(uri: Uri, timeout: Option<Duration>) -> Endpoint {
    if let Some(duration) = timeout {
        get_endpoint(uri).timeout(duration)
    } else {
        get_endpoint(uri)
    }
}

/// Fetch the env var named for `key` and parse as a `Uri`.
/// Tries the original casing, then the full lowercasing of `key`.
fn get_env_uri(key: &str) -> Result<Option<Uri>, InvalidUri> {
    std::env::var(key)
        .or_else(|_| std::env::var(key.to_lowercase()))
        .ok()
        .map(Uri::try_from)
        .transpose()
}

/// Parse the authentication from `uri` into proxy `Auth`, if present.
fn get_uri_socks_auth(uri: &Uri) -> Result<Option<Auth>, url::ParseError> {
    let full_url = uri.to_string().parse::<Url>()?;
    let user = full_url.username();
    let auth = if user.is_empty() {
        None
    } else {
        let pass = full_url.password().unwrap_or_default();
        Some(Auth::new(user, pass))
    };
    Ok(auth)
}

/// Get a [`Channel`] to the given [`Uri`].
/// Sets up things like user agent without setting up QCS credentials.
///
/// This channel will be configured to route requests through proxies defined by
/// `HTTPS_PROXY` and/or `HTTP_PROXY` environment variables, if they are defined.
/// The variable names can be all-uppercase or all-lowercase, but the all-uppercase
/// variants will take precedence. Supported proxy schemes are `http`, `https`, and `socks5`.
///
/// Proxy configuration caveats:
/// - If both variables are defined, neither can be a `socks5` proxy, unless they are both the same value.
/// - If only one variable is defined, and it is a `socks5` proxy, *all* traffic will be routed through it.
///
/// # Errors
///
/// See [`ChannelError`].
#[allow(clippy::result_large_err)]
pub fn get_channel(uri: Uri) -> Result<Channel, ChannelError> {
    let endpoint = get_endpoint(uri);
    get_channel_with_endpoint(&endpoint)
}

/// Get a [`Channel`] to the given [`Uri`], with an optional timeout. If set to [`None`], no timeout is
/// used.
/// Sets up things like user agent without setting up QCS credentials.
///
/// This channel will be configured to route requests through proxies defined by
/// `HTTPS_PROXY` and/or `HTTP_PROXY` environment variables, if they are defined.
/// The variable names can be all-uppercase or all-lowercase, but the all-uppercase
/// variants will take precedence. Supported proxy schemes are `http`, `https`, and `socks5`.
///
/// Proxy configuration caveats:
/// - If both variables are defined, neither can be a `socks5` proxy, unless they are both the same value.
/// - If only one variable is defined, and it is a `socks5` proxy, *all* traffic will be routed through it.
///
/// # Errors
///
/// See [`ChannelError`].
#[allow(clippy::result_large_err)]
pub fn get_channel_with_timeout(
    uri: Uri,
    timeout: Option<Duration>,
) -> Result<Channel, ChannelError> {
    let endpoint = get_endpoint_with_timeout(uri, timeout);
    get_channel_with_endpoint(&endpoint)
}

/// Get a [`Channel`] to the given [`Endpoint`]. Useful if [`get_channel`] or
/// [`get_channel_with_timeout`] don't provide the configurability you need.
///
/// Use [`get_endpoint`] or [`get_endpoint_with_timeout`] to get a starting
/// [`Endpoint`].
///
/// This channel will be configured to route requests through proxies defined by
/// `HTTPS_PROXY` and/or `HTTP_PROXY` environment variables, if they are defined.
/// The variable names can be all-uppercase or all-lowercase, but the all-uppercase
/// variants will take precedence. Supported proxy schemes are `http`, `https`, and `socks5`.
///
/// Proxy configuration caveats:
/// - If both variables are defined, neither can be a `socks5` proxy, unless they are both the same value.
/// - If only one variable is defined, and it is a `socks5` proxy, *all* traffic will be routed through it.
///
/// # Errors
///
/// Returns a [`ChannelError`] if the channel cannot be constructed.
#[allow(
    clippy::similar_names,
    reason = "http(s)_proxy are similar but precise in this case"
)]
#[allow(clippy::result_large_err)]
pub fn get_channel_with_endpoint(endpoint: &Endpoint) -> Result<Channel, ChannelError> {
    let https_proxy = get_env_uri("HTTPS_PROXY")?;
    let http_proxy = get_env_uri("HTTP_PROXY")?;

    let mut connector = HttpConnector::new();
    connector.enforce_http(false);

    let connect_to = |uri: http::Uri, intercept: Intercept| {
        let connector = connector.clone();
        match uri.scheme_str() {
            Some("socks5") => {
                let socks_connector = SocksConnector {
                    auth: get_uri_socks_auth(&uri)?,
                    proxy_addr: uri,
                    connector,
                };
                Ok(endpoint.connect_with_connector_lazy(socks_connector))
            }
            Some("https" | "http") => {
                let is_http = uri.scheme() == Some(&http::uri::Scheme::HTTP);
                let proxy = Proxy::new(intercept, uri);
                let mut proxy_connector = ProxyConnector::from_proxy(connector, proxy)?;
                if is_http {
                    proxy_connector.set_tls(None);
                }
                Ok(endpoint.connect_with_connector_lazy(proxy_connector))
            }
            scheme => Err(ChannelError::UnsupportedProtocol(scheme.map(String::from))),
        }
    };

    let channel = match (https_proxy, http_proxy) {
        // no proxies, default behavior
        (None, None) => endpoint.connect_lazy(),

        // either proxy may use https/http, or socks.
        (Some(https_proxy), None) => connect_to(https_proxy, Intercept::Https)?,
        (None, Some(http_proxy)) => connect_to(http_proxy, Intercept::Http)?,

        // both proxies are set. If they are the same, they can be socks5. If there are different, they
        // must both be `https?` URIs in order to use the same `ProxyConnector`.
        (Some(https_proxy), Some(http_proxy)) => {
            if https_proxy == http_proxy {
                connect_to(https_proxy, Intercept::All)?
            } else {
                let accepted = [https_proxy.scheme_str(), http_proxy.scheme_str()]
                    .into_iter()
                    .all(|scheme| matches!(scheme, Some("https" | "http")));
                if accepted {
                    let mut proxy_connector = ProxyConnector::new(connector)?;
                    proxy_connector.extend_proxies(vec![
                        Proxy::new(Intercept::Https, https_proxy),
                        Proxy::new(Intercept::Http, http_proxy),
                    ]);
                    endpoint.connect_with_connector_lazy(proxy_connector)
                } else {
                    return Err(ChannelError::Mismatch {
                        https_proxy,
                        http_proxy,
                    });
                }
            }
        }
    };

    Ok(channel)
}

/// Get a [`Channel`] to the given [`Uri`] with QCS authentication set up already.
///
/// # Errors
///
/// See [`Error`]
#[allow(clippy::result_large_err)]
pub fn get_wrapped_channel(
    uri: Uri,
) -> Result<RefreshService<Channel, ClientConfiguration>, Error<TokenError>> {
    wrap_channel(get_channel(uri)?)
}

/// Set up the given `channel` with QCS authentication.
#[must_use]
pub fn wrap_channel_with<C>(
    channel: C,
    config: ClientConfiguration,
) -> RefreshService<C, ClientConfiguration>
where
    C: GrpcService<Body>,
{
    ServiceBuilder::new()
        .layer(RefreshLayer::with_config(config))
        .service(channel)
}

/// Set up the given `channel` which will automatically
/// attempt to refresh its access token when a request fails
/// do to an expired token
pub fn wrap_channel_with_token_refresher<C, T>(
    channel: C,
    token_refresher: T,
) -> RefreshService<C, T>
where
    C: GrpcService<Body>,
    T: TokenRefresher + Clone + Send + Sync,
{
    ServiceBuilder::new()
        .layer(RefreshLayer::with_refresher(token_refresher))
        .service(channel)
}

/// Set up the given `channel` with QCS authentication.
///
/// # Errors
///
/// See [`Error`]
#[allow(clippy::result_large_err)]
pub fn wrap_channel<C>(
    channel: C,
) -> Result<RefreshService<C, ClientConfiguration>, Error<TokenError>>
where
    C: GrpcService<Body>,
{
    Ok(wrap_channel_with(channel, {
        ClientConfiguration::load_default()?
    }))
}

/// Set up the given `channel` with QCS authentication.
///
/// # Errors
///
/// See [`Error`]
#[allow(clippy::result_large_err)]
pub fn wrap_channel_with_profile<C>(
    channel: C,
    profile: String,
) -> Result<RefreshService<C, ClientConfiguration>, Error<TokenError>>
where
    C: GrpcService<Body>,
{
    Ok(wrap_channel_with(
        channel,
        ClientConfiguration::load_profile(profile)?,
    ))
}

/// Add exponential backoff retry logic to the `channel`.
pub fn wrap_channel_with_retry<C>(channel: C) -> RetryService<C>
where
    C: GrpcService<Body>,
{
    ServiceBuilder::new()
        .layer(RetryLayer::default())
        .service(channel)
}

#[cfg(feature = "tracing")]
/// Add a tracing layer with OpenTelemetry semantics to the `channel`.
pub fn wrap_channel_with_tracing(
    channel: Channel,
    base_url: String,
    configuration: TracingConfiguration,
) -> CustomTraceService {
    ServiceBuilder::new()
        .layer(build_trace_layer(base_url, Some(&configuration)))
        .service(channel)
}
