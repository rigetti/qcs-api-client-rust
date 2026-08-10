from pathlib import Path

import pytest
from qcs_api_client_common.configuration import ClientConfiguration

_DEFAULT_INTEGRATION_QPU_ID = "Ankaa-2"


def pytest_addoption(parser):
    parser.addoption(
        "--integration",
        action="store_true",
        dest="integration",
        default=False,
        help="enable integration tests (requires valid QCS configuration)",
    )

    parser.addoption(
        "--qpu",
        default=_DEFAULT_INTEGRATION_QPU_ID,
        dest="integration_qpu_id",
        help=f"the QPU ID to use for integration tests, if they are enabled (default: {_DEFAULT_INTEGRATION_QPU_ID})",
    )


def pytest_configure(config: pytest.Config):
    if not config.option.integration:
        config.option.markexpr = "not integration"


@pytest.fixture
def integration_qpu_id(request: pytest.FixtureRequest) -> str:
    cli_value = request.config.getoption("--qpu")
    return cli_value


@pytest.fixture
def fixture_env(monkeypatch) -> None:
    fixture_dir = (Path(__file__).parent / "fixtures").relative_to(Path.cwd())
    monkeypatch.setenv("QCS_SETTINGS_FILE_PATH", str(fixture_dir / "settings.toml"))
    monkeypatch.setenv("QCS_SECRETS_FILE_PATH", str(fixture_dir / "secrets.toml"))


@pytest.fixture
def empty_config_env(empty_settings_env, empty_secrets_env) -> None:
    _, _ = empty_secrets_env, empty_settings_env


@pytest.fixture
def basic_config_env(basic_settings_env, basic_secrets_env) -> None:
    _, _ = basic_secrets_env, basic_settings_env

@pytest.fixture(scope="module")
def basic_config_dir(tmp_path_factory) -> Path:
    p = tmp_path_factory.mktemp("basic_settings")
    return p


@pytest.fixture(scope="module")
def empty_config_dir(tmp_path_factory) -> Path:
    p = tmp_path_factory.mktemp("empty_settings")
    return p


@pytest.fixture(scope="module")
def basic_settings_path(basic_config_dir: Path):
    return basic_config_dir / "settings.toml"


@pytest.fixture(scope="module")
def basic_secrets_path(basic_config_dir: Path) -> Path:
    return basic_config_dir / "secrets.toml"


@pytest.fixture(scope="module")
def empty_settings_path(empty_config_dir: Path):
    return empty_config_dir / "settings.toml"


@pytest.fixture(scope="module")
def empty_secrets_path(empty_config_dir: Path) -> Path:
    return empty_config_dir / "secrets.toml"


@pytest.fixture(scope="session")
def mock_url() -> str:
    return "http://example.com"


@pytest.fixture(scope="module", autouse=True)
def files(
    basic_settings_path: Path,
    basic_secrets_path: Path,
    empty_settings_path: Path,
    empty_secrets_path: Path,
    mock_url: str,
) -> None:
    basic_config = rf"""
default_profile_name = "staging1"

[profiles]

    [profiles.staging1]
    auth_server_name = "staging1"
    api_url = "{mock_url}"
    grpc_api_url = "{mock_url}"
    credentials_name = "staging1"

        [profiles.staging1.applications]
        cli = {{verbosity = "info"}}

    [profiles.staging2]
    auth_server_name = "staging2"
    api_url = "{mock_url}"
    credentials_name = "staging2"

        [profiles.staging2.applications]
        cli = {{verbosity = "debug"}}

    [profiles.missing_auth_server]
    auth_server_name = "doesntexist"
    api_url = "{mock_url}"

[auth_servers]
    [auth_servers.staging1]
    client_id = "0oarkug104njPxvTZ0h7"
    issuer = "https://auth.staging.qcs.rigetti.com/oauth2/ausg5h11bo5xyWoP30h7"

    [auth_server.staging2]
    client_id = "abcdefghijklmnopqrstuvwxyz"
    issuer = "https://us-east-abcdefghij.auth.us-east-2.amazoncognito.com"
    scopes = ["email", "openid", "profile", "openid email"]
"""

    basic_secrets = """
[credentials]

  [credentials.default]
    [credentials.default.token_payload]
      access_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6ImRlZmF1bHQiLCJpYXQiOjE2MDU1NTMyNjB9.vBYtPsolKPa_KTbfiVrOAqV9s_-oXMPHK59zyj41g0Y"
      refresh_token = "refresh"
      token_type = "Bearer"

  [credentials.staging1]
    [credentials.staging1.token_payload]
      access_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6ImNvdyIsImlhdCI6MTYwNTU1MzI2MH0.lRhv60Z5iN0w1g3uwDDi-cNAI4-qLIFkBCe-2__PSVo"
      refresh_token = "refresh1"
      token_type = "Bearer"

  [credentials.staging2]
    [credentials.staging2.token_payload]
      access_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6ImRvZyIsImlhdCI6MTYwNTU1MzI2MH0.hTK7lzCF5VZAqsJBTyzacm76PSFeuiKi9vAwb7lw6DE"
      refresh_token = "refresh2"
      token_type = "Bearer"
"""

    basic_settings_path.write_text(basic_config)
    basic_secrets_path.write_text(basic_secrets)
    empty_settings_path.write_text("")
    empty_secrets_path.write_text("")


@pytest.fixture
def basic_secrets_env(monkeypatch, basic_secrets_path: Path) -> None:
    monkeypatch.setenv("QCS_SECRETS_FILE_PATH", str(basic_secrets_path))


@pytest.fixture
def basic_settings_env(monkeypatch, basic_settings_path: Path) -> None:
    monkeypatch.setenv("QCS_SETTINGS_FILE_PATH", str(basic_settings_path))


@pytest.fixture
def empty_secrets_env(monkeypatch, empty_secrets_path: Path) -> None:
    monkeypatch.setenv("QCS_SECRETS_FILE_PATH", str(empty_secrets_path))


@pytest.fixture
def empty_settings_env(monkeypatch, empty_settings_path: Path) -> None:
    monkeypatch.setenv("QCS_SETTINGS_FILE_PATH", str(empty_settings_path))

@pytest.fixture
def client_configuration() -> ClientConfiguration:
    return ClientConfiguration.load_default()


