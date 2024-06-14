"""QCS Middleware for requests made with `httpx`."""

from typing import Optional

import httpx
from httpx import Request

from qcs_api_client_common.configuration import ClientConfiguration


class QCSAuthorization(httpx.Auth):
    """An `httpx.Auth` that will add QCS authorization to your requests and refresh the token as needed.

    Synchronous usage:

    ```python
    import httpx

    auth = QCSAuthorization()
    client = httpx.Client(auth=auth)

    response = client.get('https://rigetti.com/protected-resource')
    ```

    Asynchronous usage:

    ```python
    import httpx
    import asyncio

    async def main():
        auth = QCSAuthorization()
        async with httpx.AsyncClient(auth=auth) as client:
            response = await client.get('https://rigetti.com/protected-resource')
            print(response.status_code)
            print(response.json())

    asyncio.run(main())
    ```
    """
    def __init__(self, client_configuration: Optional[ClientConfiguration] = None):
        """Initialize the authorization with an optional client configuration.

        If `client_configuration` is set to None, `ClientConfiguration.load_default()` will be used.
        """
        self.configuration = client_configuration or ClientConfiguration.load_default()

    # Note: No explicit type hints here. It confuses stubtest, and they are inherited from the base class anyways.
    async def async_auth_flow(self, request: Request):
        """Add the QCS authorization token to the request, refreshing the token as needed."""
        token = await self.configuration.get_bearer_access_token_async()
        request.headers['Authorization'] = f'Bearer {token}'
        yield request


    def sync_auth_flow(self, request: Request):
        """Add the QCS authorization token to the request, refreshing the token as needed."""
        token = self.configuration.get_bearer_access_token()
        request.headers['Authorization'] = f'Bearer {token}'
        yield request
