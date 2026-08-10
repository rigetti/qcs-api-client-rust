"""Guards on the public package namespace.

Unlike ``qcs_api_client_common/configuration/__init__.py``, the top-level
``qcs_api_client_common/__init__.py`` is hand-written (it also exposes the
pure-Python ``grpc`` and ``httpx`` modules), so it can drift from the compiled
module. These tests fail when it does.
"""

import qcs_api_client_common
from qcs_api_client_common import _qcs_api_client_common


def test_all_matches_compiled_module():
    """The public namespace is the compiled module's, plus the pure-Python modules."""
    assert sorted(qcs_api_client_common.__all__) == sorted([*_qcs_api_client_common.__all__, "grpc", "httpx"])


def test_all_is_importable():
    """Every name in ``__all__`` is actually bound on the package."""
    for name in qcs_api_client_common.__all__:
        assert hasattr(qcs_api_client_common, name), f"{name} is in __all__ but not exported"
