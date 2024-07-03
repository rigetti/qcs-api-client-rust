import os

import pytest

_DEFAULT_INTEGRATION_QPU_ID = "Ankaa-2"

os.environ["QCS_SETTINGS_FILE_PATH"] = os.path.join(".", "tests_py", "fixtures", "settings.toml")
os.environ["QCS_SECRETS_FILE_PATH"] = os.path.join(".", "tests_py", "fixtures", "secrets.toml")


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
def integration_qpu_id(request: pytest.Config) -> str:
    cli_value = request.config.getoption("--qpu")
    return cli_value

