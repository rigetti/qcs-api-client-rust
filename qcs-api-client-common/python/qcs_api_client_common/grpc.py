"""Middleware for gRPC clients."""

from abc import ABCMeta
from collections.abc import Awaitable, Callable
from typing import TypeVar

import grpc
from grpc.aio import ClientCallDetails, Metadata, UnaryUnaryCall, UnaryUnaryClientInterceptor

from .configuration import ClientConfiguration, SecretAccessToken

_RequestType = TypeVar("_RequestType")
_ResponseType = TypeVar("_ResponseType")


class RefreshInterceptor(UnaryUnaryClientInterceptor, metaclass=ABCMeta):
    """A `RefreshInterceptor` will add your QCS authorization token to all your QCS requests.

    The interceptor will automatically refresh your token as needed.
    """

    def __init__(self, client_configuration: ClientConfiguration | None = None):
        """Initialize the interceptor using a `ClientConfiguration`.

        If `client_configuration` is unset, `ClientConfiguratio.load_default()` will be used.
        """
        if client_configuration is None:
            client_configuration = ClientConfiguration.load_default()
        self.configuration = client_configuration

    async def _get_access_token(self) -> SecretAccessToken:
        return await self.configuration.get_bearer_access_token_async()

    async def intercept_unary_unary(
        self,
        continuation: Callable[
            [ClientCallDetails, _RequestType], Awaitable[UnaryUnaryCall[_RequestType, _ResponseType]]
        ],
        client_call_details: ClientCallDetails,
        request: _RequestType,
    ) -> UnaryUnaryCall[_RequestType, _ResponseType]:
        """Adds the QCS authorization token to the request metadata, refreshing the token if needed."""
        # Copy any existing headers and set the access token.
        # `Metadata` iterates as `(key, value)` pairs, so this also tolerates the plain sequence of
        # pairs that `ClientCallDetails` permits, even though `grpc.aio` normalizes the caller's
        # metadata to `Metadata` before any interceptor runs.
        authorized_metadata = Metadata(*(client_call_details.metadata or ()))
        authorized_metadata["authorization"] = f"Bearer {await self._get_access_token()}"

        new_client_call_details = grpc.aio.ClientCallDetails(
            client_call_details.method,
            client_call_details.timeout,
            authorized_metadata,
            client_call_details.credentials,
            client_call_details.wait_for_ready,
        )

        return await continuation(new_client_call_details, request)
