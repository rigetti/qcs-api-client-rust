import pytest
from qcs_api_client_common import QcsApiClientError
from qcs_api_client_common.configuration import (
    AuthServer,
    ClientConfiguration,
    ExternallyManaged,
    OAuthSession,
    RefreshToken,
    SecretAccessToken,
    SecretRefreshToken,
    TokenError,
)
from syrupy.assertion import SnapshotAssertion


class TestClientConfiguration:
    @pytest.mark.usefixtures("fixture_env")
    def test_default(self, snapshot: SnapshotAssertion):
        cc = ClientConfiguration.load_default()
        assert cc == snapshot

    @pytest.mark.usefixtures("fixture_env")
    def test_load_profile(self, snapshot: SnapshotAssertion):
        cc = ClientConfiguration.load_profile("test")
        assert cc == snapshot

    @pytest.mark.usefixtures("basic_config_env")
    def test_initialization(self, client_configuration: ClientConfiguration, mock_url: str):
        """Assert that the client can load settings and secrets from respective files."""

        assert client_configuration.api_url == mock_url
        assert client_configuration.grpc_api_url == mock_url

        oauth_session = client_configuration.get_oauth_session()

        assert oauth_session.auth_server.client_id == "0oarkug104njPxvTZ0h7"

        assert isinstance(oauth_session.payload, RefreshToken)
        assert oauth_session.payload.refresh_token.secret == "refresh1"

        assert isinstance(oauth_session.access_token, SecretAccessToken)
        assert (
            oauth_session.access_token.secret
            == "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6ImNvdyIsImlhdCI6MTYwNTU1MzI2MH0.lRhv60Z5iN0w1g3uwDDi-cNAI4-qLIFkBCe-2__PSVo"
        )

    @pytest.mark.usefixtures("basic_config_env")
    def test_config_missing_auth_server(self):
        """Assert an error is raised if the auth server does not exist."""

        with pytest.raises(QcsApiClientError):
            _ = ClientConfiguration.load_profile("missing_auth_server")

    @pytest.mark.usefixtures("empty_config_env")
    def test_config_missing_profile(self):
        """Assert an error is raised if the profile does not exist."""

        with pytest.raises(QcsApiClientError):
            _ = ClientConfiguration.load_profile("doesnt_exist")

    @pytest.mark.usefixtures("empty_settings_env")
    @pytest.mark.usefixtures("basic_secrets_env")
    def test_empty_settings_file(self):
        """Assert defaults populated if settings file is empty."""

        client_configuration = ClientConfiguration.load_default()
        oauth_session = client_configuration.get_oauth_session()
        auth_server = oauth_session.auth_server

        assert isinstance(oauth_session.payload, RefreshToken)
        refresh_token = oauth_session.payload.refresh_token

        assert client_configuration.grpc_api_url is not None
        assert client_configuration.grpc_api_url != ""

        assert auth_server.client_id is not None
        assert auth_server.client_id != ""

        assert auth_server.issuer is not None
        assert auth_server.issuer != ""

        assert refresh_token.secret == "refresh"

    @pytest.mark.usefixtures("basic_config_env")
    def test_env_overrides(self, monkeypatch):
        """Assert that certain values can be overridden via environment variables."""
        from qcs_api_client_common import configuration

        monkeypatch.setenv(configuration.API_URL_VAR, "http://api.mock")
        monkeypatch.setenv(configuration.GRPC_API_URL_VAR, "http://grpc.mock")
        monkeypatch.setenv(configuration.QUILC_URL_VAR, "http://quilc.mock")
        monkeypatch.setenv(configuration.QVM_URL_VAR, "http://qvm.mock")

        client_configuration = ClientConfiguration.load_default()

        assert client_configuration.api_url == "http://api.mock"
        assert client_configuration.grpc_api_url == "http://grpc.mock"
        assert client_configuration.quilc_url == "http://quilc.mock"
        assert client_configuration.qvm_url == "http://qvm.mock"

    @pytest.mark.usefixtures("basic_settings_env")
    @pytest.mark.usefixtures("empty_secrets_env")
    def test_secrets_file_is_empty(self, mock_url: str):
        """Assert defaults loaded if secrets file is empty."""

        client_configuration = ClientConfiguration.load_profile("staging1")

        assert client_configuration.api_url == mock_url
        assert client_configuration.grpc_api_url == mock_url

        with pytest.raises(QcsApiClientError):
            _ = client_configuration.get_oauth_session()


class TestClientConfigurationBuilder:
    def test_build(self, snapshot: SnapshotAssertion):
        builder = ClientConfiguration.builder()
        builder.api_url = "builder_api_url"
        builder.grpc_api_url = "builder_grpc_api_url"
        builder.quilc_url = "builder_quilc_url"
        builder.qvm_url = "builder_qvm_url"
        auth_server = AuthServer("builder_client_id", "builder_issuer")
        builder.oauth_session = OAuthSession(RefreshToken(SecretRefreshToken("builder_refresh_token")), auth_server, SecretAccessToken("builder_access_token"))
        assert builder.build() == snapshot


class TestAuthServer:
    def test_properties(self):
        auth_server = AuthServer("client_id", "issuer")
        assert auth_server.client_id == "client_id"
        assert auth_server.issuer == "issuer"

    def test_eq(self):
        auth_server = AuthServer("client_id", "issuer")
        assert auth_server == auth_server
        assert auth_server == AuthServer("client_id", "issuer")
        assert auth_server != AuthServer("different_client_id", "issuer")
        assert auth_server != AuthServer("client_id", "different_issuer")


class TestCredentials:
    def test_properties(self):
        payload = RefreshToken(SecretRefreshToken("refresh"))
        auth_server = AuthServer("some_client_id", "some_issuer")
        credentials = OAuthSession(payload, auth_server, SecretAccessToken("access"))
        assert isinstance(credentials.access_token, SecretAccessToken)
        assert credentials.access_token.secret == "access"
        assert credentials.auth_server == auth_server
        assert credentials.payload == payload


class TestOAuthSession:
    def test_externally_managed(self):
        expected_auth_server = AuthServer("client_id", "issuer")

        def refresh_function(auth_server: AuthServer):
            assert auth_server == expected_auth_server
            return "access_token_from_refresh_function"

        manager = ExternallyManaged(refresh_function)

        session = OAuthSession(manager, expected_auth_server)

        token = session.request_access_token()
        assert token.secret == "access_token_from_refresh_function"


class TestSecrets:
    def test_secret_access_token(self):
        secret = "super_secret"
        secret_access_token = SecretAccessToken(secret)
        assert secret not in repr(secret_access_token)
        assert secret_access_token.secret == secret

    def test_secret_refresh_token(self):
        secret = "super_secret"
        secret_refresh_token = SecretRefreshToken(secret)
        assert secret not in repr(secret_refresh_token)
        assert secret_refresh_token.secret == secret


@pytest.mark.asyncio
async def test_sync_method_from_async_context():
    """
    Simulates calling a synchronous method from JupyterHub, which has an already-running event loop.
    """
    cc = ClientConfiguration.load_default()

    # Call multiple times to make sure that the mechanism works for repeated calls.
    for _ in range(10):
        _ = cc.get_oauth_session()

    # Ensure that errors are propagated as exceptions.
    with pytest.raises(TokenError):
        ClientConfiguration().get_oauth_session()

    # ...and that this does not prevent further calls.
    _ = cc.get_oauth_session()

