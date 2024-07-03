from unittest.mock import AsyncMock, Mock

import httpx
import pytest

from qcs_api_client_common.configuration import ClientConfiguration
from qcs_api_client_common.httpx import QCSAuthorization


@pytest.fixture
def mock_client_config():
    config = AsyncMock(spec=ClientConfiguration)
    config.get_bearer_access_token = Mock(return_value='mock_client_token')
    config.get_bearer_access_token_async = AsyncMock(return_value='mock_client_token')
    return config

def test_auth_sync(mock_client_config):
    request = httpx.Request("GET", "https://rigetti.com")
    auth = QCSAuthorization(mock_client_config).sync_auth_flow(request)

    try:
        next(auth)
    except StopIteration:
        pass

    assert request.headers["Authorization"] == "Bearer mock_client_token"
    mock_client_config.get_bearer_access_token.assert_called_once()

@pytest.mark.asyncio
async def test_auth_async(mock_client_config):
    request = httpx.Request("GET", "https://example.com")
    auth = QCSAuthorization(mock_client_config).async_auth_flow(request)

    try:
        await auth.asend(None)
    except StopAsyncIteration:
        pass

    assert request.headers["Authorization"] == "Bearer mock_client_token"
    mock_client_config.get_bearer_access_token_async.assert_called_once()
