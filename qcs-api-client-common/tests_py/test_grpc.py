import asyncio
from unittest.mock import MagicMock

import grpc
import pytest
from google.protobuf.any_pb2 import Any

from qcs_api_client_common.configuration import ClientConfiguration
from qcs_api_client_common.grpc import RefreshInterceptor


@pytest.fixture
async def interceptor() -> RefreshInterceptor:
    return RefreshInterceptor(None)

@pytest.fixture
def client_call_details() -> grpc.aio.ClientCallDetails:
    method = '/test.TestService/TestMethod'
    timeout = None
    metadata = [('initial', 'metadata')]
    credentials = None
    wait_for_ready = None
    return grpc.aio.ClientCallDetails(method, timeout, metadata, credentials, wait_for_ready)

@pytest.fixture
def make_request() -> Any:
    return Any()

@pytest.fixture
def mock_config(mocker):
    mock = mocker.patch.object(ClientConfiguration, 'get_bearer_access_token_async', new_callable=MagicMock)
    future = asyncio.Future()
    future.set_result("mock_config_fixture_token")
    mock.return_value = future

    return mock

@pytest.mark.asyncio
async def test_refresh_interceptor(mock_config, client_call_details, make_request):
    """Test that the interceptor properly adds the authorization metadata."""

    async def continuation(call_details: grpc.aio.ClientCallDetails, request: Any):
        return call_details

    updated_call_details = await RefreshInterceptor().intercept_unary_unary(continuation, client_call_details, make_request)

    # Verify that the metadata now includes the authorization token
    assert ('authorization', 'Bearer mock_config_fixture_token') in updated_call_details.metadata, "Authorization token is missing in metadata"
    assert ('initial', 'metadata') in updated_call_details.metadata, "Original metadata is not preserved"
