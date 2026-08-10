"""A suite of common functionalities for QCS client applications.

This package offers reusable middleware implementations
that can be integrated into various client libraries.
This allows for consistent behavior across different projects
and facilitates easier maintenance and scalability of client-side logic.

⚠️ This package is still in early development
and breaking changes should be expected between minor versions.
"""

# The following code exposes the package contents under the expected namespace.
from . import _qcs_api_client_common
from . import grpc, httpx

assert isinstance(_qcs_api_client_common.__all__, list) and all(isinstance(s, str) for s in _qcs_api_client_common.__all__)
exec(
    f"from ._qcs_api_client_common import {', '.join(_qcs_api_client_common.__all__)}; "
    f"__all__ = {_qcs_api_client_common.__all__} + ['grpc', 'httpx']"
)
del _qcs_api_client_common

