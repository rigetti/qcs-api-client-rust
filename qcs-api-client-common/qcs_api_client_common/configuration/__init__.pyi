from typing import Callable, final

__all__ = [
    "API_URL_VAR",
    "AuthServer",
    "ClientConfiguration",
    "ClientConfigurationBuilder",
    "ClientCredentials",
    "ClientSecret",
    "DEFAULT_API_URL",
    "DEFAULT_GRPC_API_URL",
    "DEFAULT_PROFILE_NAME",
    "DEFAULT_QUILC_URL",
    "DEFAULT_QVM_URL",
    "DEFAULT_SECRETS_PATH",
    "DEFAULT_SETTINGS_PATH",
    "ExternallyManaged",
    "GRPC_API_URL_VAR",
    "OAuthSession",
    "PROFILE_NAME_VAR",
    "QUILC_URL_VAR",
    "QVM_URL_VAR",
    "RefreshToken",
    "PkceFlow",
    "SecretAccessToken",
    "SecretRefreshToken",
    "SECRETS_PATH_VAR",
    "SETTINGS_PATH_VAR",
]

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
    def load_default_with_login() -> ClientConfiguration:
        """Similar to `ClientConfiguration#load_default`, but will attempt a browser redirect login if the stored credentials are not available or invalid. Avoid using this in hosted (non-local) Jupyter notebook environments."""
        ...

    @staticmethod
    def load_profile(profile_name: str) -> ClientConfiguration:
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

    def get_bearer_access_token(self) -> SecretAccessToken:
        """Gets the `Bearer` access token, refreshing it if is expired."""

    async def get_bearer_access_token_async(self) -> SecretAccessToken:
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
    def __new__(cls, client_id: str, issuer: str, scopes: list[str] | None = None) -> AuthServer:
        """
        Create a new AuthServer.

        Args:
            client_id: The client's OAuth OIDC client ID.
            issuer: The OAuth OIDC issuer URL.
            scopes: Optional list of scopes to request when requesting authorization tokens. If None, all scopes from the issuer's discovery document will be used.
        """

    @staticmethod
    def default() -> AuthServer:
        """Get the default OAuth OIDC auth server."""

    @property
    def client_id(self) -> str:
        """The client's OAuth OIDC client ID."""

    @property
    def issuer(self) -> str:
        """The OAuth OIDC issuer URL."""

    @property
    def scopes(self) -> list[str] | None:
        """
        The list of scopes to request when requesting authorization tokens, if explicitly configured.

        Note that this property does not return the supported scopes from the issuer's discovery document in the case where scopes were not explicitly configured. 
        """

@final
class RefreshToken:
    def __new__(cls, refresh_token: str) -> RefreshToken: ...
    @property
    def refresh_token(self) -> SecretRefreshToken:
        """The refresh token."""
    @refresh_token.setter
    def refresh_token(self, refresh_token: SecretRefreshToken):
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
class ExternallyManaged:
    def __new__(cls, refresh_function: Callable[[AuthServer], str]) -> ExternallyManaged:
        """Manages access tokens by utilizing a user-provided refresh function.

        The refresh function should return a valid access token, or raise an exception if it cannot.

        .. testcode::
            from qcs_apiclient_common.configuration import AuthServer, ExternallyManaged, OAuthSession

            def refresh_function(auth_server: AuthServer) -> str:
                return "my_access_token"

            externally_managed = ExternallyManaged(refresh_function)
            session = OAuthSession(externally_managed, AuthServer.default())
        """

@final
class PkceFlow:
    """Represents a PKCE flow. This will automatically initiate a browser redirect flow when constructed. Avoid using this in hosted (non-local) Jupyter notebook environments."""
    def __new__(cls, auth_server: AuthServer) -> PkceFlow: ...
    @property
    def access_token(self) -> SecretAccessToken:
        """The access token."""
    @property
    def refresh_token(self) -> SecretRefreshToken | None:
        """The refresh token."""

@final
class OAuthSession:
    def __new__(
        cls,
        payload: RefreshToken | ClientCredentials | ExternallyManaged | PkceFlow,
        auth_server: AuthServer,
        access_token: SecretAccessToken | None = None,
    ) -> OAuthSession: ...

    @property
    def access_token(self) -> SecretAccessToken:
        """Get the current access token.

        This is an unvalidated copy of the access token. Meaning it can become stale, or may already be stale. See the `validate` `request_access_token` and methods.
        """

    @property
    def auth_server(self) -> AuthServer:
        """The auth server."""

    @property
    def payload(self) -> RefreshToken | ClientCredentials | ExternallyManaged | PkceFlow:
        """Get the payload used to request an access token."""

    def request_access_token(self) -> SecretAccessToken:
        """Request a new access token."""

    async def request_access_token_async(self) -> SecretAccessToken:
        """Request a new access token."""

    def validate(self) -> SecretAccessToken:
        """Validate the current access token, returning it if it is valid.

        If the token is invalid, a `ValueError` will be raised with a description of why the token failed validation.
        """


@final
class SecretAccessToken:
    """A secret access token, which redacts the sensitive value when printed. The actual value can be retrieved using the `secret` property."""

    def __new__(cls, value: str) -> SecretAccessToken: ...
    
    @property
    def is_empty(self) -> bool:
        """Check if the access token is empty."""
    @property
    def secret(self) -> str:
        """CAUTION: Take care not to reveal this value to untrusted parties, as it can be used to authenticate on your behalf!"""

@final
class SecretRefreshToken:
    """A secret refresh token, which redacts the sensitive value when printed. The actual value can be retrieved using the `secret` property."""

    def __new__(cls, value: str) -> SecretRefreshToken: ...
    @property
    def is_empty(self) -> bool:
        """Check if the access token is empty."""
    @property
    def secret(self) -> str:
        """CAUTION: Take care not to reveal this value to untrusted parties, as it can be used to authenticate on your behalf!"""


@final
class ClientSecret:
    """A client secret, which redacts the sensitive value when printed. The actual value can be retrieved using the `secret` property."""

    def __new__(cls, value: str) -> ClientSecret: ...
    @property
    def is_empty(self) -> bool:
        """Check if the client secret is empty."""
    @property
    def secret(self) -> str:
        """CAUTION: Take care not to reveal this value to untrusted parties, as it can be used to authenticate on your behalf!"""