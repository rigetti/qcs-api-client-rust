"""Middleware for gRPC clients."""
from abc import ABCMeta
from typing import Any, Callable, List, Optional, Tuple

import grpc  # type: ignore
from grpc import ClientCallDetails  # type: ignore
from grpc.aio import Call, UnaryUnaryClientInterceptor  # type: ignore

from qcs_api_client_common.configuration import ClientConfiguration


class RefreshInterceptor(UnaryUnaryClientInterceptor, metaclass=ABCMeta):
    """A `RefreshInterceptor` will add your QCS authorization token to all your QCS requests.

    The interceptor will automatically refresh your token as needed.
    """
    def __init__(self, client_configuration: Optional[ClientConfiguration]=None):
        """Initialize the interceptor using a `ClientConfiguration`.

        If `client_configuration` is unset, `ClientConfiguratio.load_default()` will be used.
        """
        if client_configuration is None:
            client_configuration = ClientConfiguration.load_default()
        self.configuration = client_configuration

    async def _get_access_token(self) -> str:
        return await self.configuration.get_bearer_access_token_async()

    async def intercept_unary_unary(
        self,
        continuation: Callable[[ClientCallDetails, Any], Call],
        client_call_details: ClientCallDetails,
        request: Any
    ) -> Any:
        """Adds the QCS authorization token to the request metadata, refreshing the token if needed."""
        # Modify the headers to add the access token
        authorized_metadata: List[Tuple[str, str]] = []
        if client_call_details.metadata is not None:
            authorized_metadata = list(client_call_details.metadata)

        authorized_metadata.append(('authorization', f'Bearer {await self._get_access_token()}'))

        new_client_call_details = grpc.aio.ClientCallDetails(
            client_call_details.method,
            client_call_details.timeout,
            authorized_metadata,
            client_call_details.credentials,
            client_call_details.wait_for_ready,
        )

        return await continuation(new_client_call_details, request)
