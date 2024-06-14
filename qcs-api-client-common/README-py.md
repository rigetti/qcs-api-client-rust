# QCS API Client Common

The `qcs-api-client-common` package provides a suite of common functionalities for
QCS client applications. It offers reusable middleware implementations that can be
integrated into various client libraries. This allows for consistent behavior
across different projects and facilitates easier maintenance and scalability
of client-side logic.

⚠️ This project is in-development.

## Loading a QCS Configuration

The primary configuration object is the `ClientConfiguration`. Most users will
want to load their default profile like so:

```python
from qcs_api_client_common.configuration import ClientConfiguration

configuration = ClientConfiguration()
# ...
```

For more details and other examples of usage, see the `ClientConfiguration`
documentation.

## Supported clients

🔧 Under Construction
