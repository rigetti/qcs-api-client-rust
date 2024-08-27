from typing import final

DEFAULT_API_URL: str
DEFAULT_GRPC_API_URL: str
DEFAULT_QUILC_URL: str
DEFAULT_QVM_URL: str
DEFAULT_PROFILE_NAME: str
PROFILE_NAME_VAR: str
QUILC_URL_VAR: str
QVM_URL_VAR: str
API_URL_VAR: str
GRPC_API_URL_VAR: str
SETTINGS_PATH_VAR: str
SECRETS_PATH_VAR: str
DEFAULT_SETTINGS_PATH: str
DEFAULT_SECRETS_PATH: str

@final
class ClientConfiguration:
    """A configuration suitable for use as a QCS API Client.

    This configuration can be constructed in a few ways.

    The most common way is to use `ClientConfiguration#load_default`. This will load the
    configuration associated with your default QCS profile. When loading a profile, any values set by environment variables will override the values in your configuration files.

    You can also build a configuration from scratch using `ClientConfigurationBuilder`. Using a
    builder bypasses configuration files and environment overrides.
    """
    @staticmethod
    def load_default() -> ClientConfiguration:
        """Load a `ClientConfiguration` using your default QCS profile."""
        ...

    @staticmethod
    def load_profile(profile_name: str) -> ClientConfigurationBuilder:
        """Load a `ClientConfiguration` using the given QCS profile."""

    @staticmethod
    def builder() -> ClientConfigurationBuilder:
        """Create a new `ClientConfigurationBuilder`."""

    @property
    def api_url(self) -> str:
        """The URL of the QCS REST API."""

    @property
    def grpc_api_url(self) -> str:
        """The URL of the QCS gRPC API."""

    @property
    def quilc_url(self) -> str:
        """The URL of the QCS quilc API."""

    @property
    def qvm_url(self) -> str:
        """The URL of the QCS QVM API."""

    def get_oauth_session(self) -> OAuthSession:
        """Get the credentials used to authenticate with the QCS API."""

    async def get_oauth_session_async(self) -> OAuthSession:
        """Get the credentials used to authenticate with the QCS API."""

    def get_bearer_access_token(self) -> str:
        """Gets the `Bearer` access token, refreshing it if is expired."""

    async def get_bearer_access_token_async(self) -> str:
        """Gets the `Bearer` access token, refreshing it if is expired."""

@final
class ClientConfigurationBuilder:
    def __new__(cls) -> ClientConfigurationBuilder: ...
    def build(self) -> ClientConfiguration:
        """Build a `ClientConfiguration` using the values provided to this builder."""
        ...

    @property
    def api_url(self):
        raise AttributeError("api_url is not readable")

    @api_url.setter
    def api_url(self, api_url: str):
        """Set the URL to use for the QCS REST API."""

    @property
    def grpc_api_url(self):
        raise AttributeError("grpc_api_url is not readable")

    @grpc_api_url.setter
    def grpc_api_url(self, grpc_api_url: str):
        """Set the URL to use for the QCS gRPC API."""

    @property
    def quilc_url(self):
        raise AttributeError("quilc_url is not readable")

    @quilc_url.setter
    def quilc_url(self, quilc_url: str):
        """Set the URL to use for the quilc server."""

    @property
    def qvm_url(self):
        raise AttributeError("qvm_url is not readable")

    @qvm_url.setter
    def qvm_url(self, qvm_url: str):
        """Set the URL to use for the QVM server."""

    @property
    def oauth_session(self):
        raise AttributeError("tokens is not readable")

    @oauth_session.setter
    def oauth_session(self, tokens: OAuthSession | None):
        """Set the QCS API access and refresh `Credentials` to use."""

@final
class AuthServer:
    def __new__(cls, client_id: str, issuer: str) -> AuthServer: ...
    @staticmethod
    def default() -> AuthServer:
        """Get the default Okta auth server."""

    @property
    def client_id(self) -> str:
        """The client's Okta ID."""

    @property
    def issuer(self) -> str:
        """The Okta issuer URL."""

@final
class RefreshToken:
    def __new__(cls, refresh_token: str) -> RefreshToken: ...
    @property
    def refresh_token(self) -> str:
        """The refresh token."""
    @refresh_token.setter
    def refresh_token(self, refresh_token: str):
        """Set the refresh token."""

@final
class ClientCredentials:
    def __new__(cls, client_id: str, client_secret: str) -> ClientCredentials: ...
    @property
    def client_id(self) -> str:
        """The client ID."""
    @property
    def client_secret(self) -> str:
        """The client secret."""

@final
class OAuthSession:
    def __new__(
        cls, grant_payload: RefreshToken | ClientCredentials, auth_server: AuthServer, access_token: str | None = None
    ) -> OAuthSession: ...
    @property
    def access_token(self) -> str:
        """Get the current access token.

        This is an unvalidated copy of the access token. Meaning it can become stale, or may already be stale. See the `validate` `request_access_token` and methods.
        """

    @property
    def auth_server(self) -> AuthServer:
        """The refresh token."""

    @property
    def payload(self) -> RefreshToken | ClientCredentials:
        """Get the payload used to request an access token."""

    def validate(self) -> str:
        """Validate the current access token, returning it if it is valid.

        If the token is invalid, a `ValueError` will be raised with a description of why the token failed validation.
        """
