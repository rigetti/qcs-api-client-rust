from syrupy.assertion import SnapshotAssertion

from qcs_api_client_common.configuration import AuthServer, ClientConfiguration, OAuthSession, RefreshToken


class TestClientConfiguration:
    def test_default(self, snapshot: SnapshotAssertion):
        cc = ClientConfiguration.load_default()
        assert cc == snapshot

    def test_load_profile(self, snapshot: SnapshotAssertion):
        cc = ClientConfiguration.load_profile("test")
        assert cc == snapshot


class TestClientConfigurationBuilder:
    def test_build(self, snapshot: SnapshotAssertion):
        builder = ClientConfiguration.builder()
        builder.api_url = "builder_api_url"
        builder.grpc_api_url = "builder_grpc_api_url"
        builder.quilc_url = "builder_quilc_url"
        builder.qvm_url = "builder_qvm_url"
        auth_server = AuthServer("builder_client_id", "builder_issuer")
        builder.oauth_session = OAuthSession(RefreshToken("builder_refresh_token"), auth_server, "builder_access_token")
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
        payload = RefreshToken("refresh")
        auth_server = AuthServer("some_client_id", "some_issuer")
        credentials = OAuthSession(payload, auth_server, "access")
        assert credentials.access_token == "access"
        assert credentials.auth_server == auth_server
        assert credentials.payload == payload

    def test_eq(self):
        payload = RefreshToken("refresh")
        auth_server = AuthServer("some_client_id", "some_issuer")
        credentials = OAuthSession(payload, auth_server, "access")
        assert credentials == credentials
        assert credentials == OAuthSession(payload, auth_server, "access")
        assert credentials != OAuthSession(payload, auth_server, "different_access")
        assert credentials != OAuthSession(RefreshToken("different_refresh"), auth_server, "access")
