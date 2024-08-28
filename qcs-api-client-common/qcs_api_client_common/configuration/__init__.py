"""This module facilitates loading a QCS configuration to connect to real QPUs or the QVM.

The primary class provided is :class:`ClientConfiguration`, which allows you to load and access your configuration.
You can load the configuration from disk, typically from the ``~/.qcs`` directory (recommended for most users), or programmatically using
:class:`ClientConfigurationBuilder`.

### Loading Configuration from Disk

Most users will only need to use :meth:`ClientConfiguration.load_default`. This method instantiates a new
:class:`ClientConfiguration` with settings loaded from files in the ``~/.qcs`` directory, including:

* ``settings.toml``: General settings, such as connection URLs.
* ``secrets.toml``: Authentication tokens.

Both files should contain profiles. The ``default_profile_name`` setting determines which profile is loaded by default.
To load a non-default profile, use :meth:`ClientConfiguration.load_profile`.

For details on obtaining these files, refer to the `QCS Credentials Guide <https://docs.rigetti.com/qcs/guides/qcs-credentials>`_.

### Using the ClientConfigurationBuilder

:class:`ClientConfigurationBuilder` allows for programmatically building a :class:`ClientConfiguration`. This is useful
for loading configurations from alternative sources or overriding specific settings. Advanced functionality, such as
using a different OAuth grant type, is only available through the builder.

Example usage:

.. code-block:: python

    from qcs_api_client_common.configuration import ClientConfiguration, OAuthSession, ClientCredentials

    # Initialize a builder
    builder = ClientConfiguration.builder()

    # Set custom settings, e.g., a custom OAuth session
    oauth = OAuthSession(ClientCredentials("client_id", "client_secret"), AuthServer.default())
    builder.oauth_session(oauth)

    # Build and retrieve the configuration
    configuration = builder.build()

### Environment Variables

You can override configuration values using environment variables, which take precedence over values loaded from files
but are overridden by settings explicitly set in a builder.

* ``QCS_SETTINGS_FILE_PATH``: Path to the ``settings.toml`` file.
* ``QCS_SECRETS_FILE_PATH``: Path to the ``secrets.toml`` file.
* ``QCS_PROFILE_NAME``: Profile to load from ``settings.toml``.
* ``QCS_SETTINGS_APPLICATIONS_QUILC_URL``: URL for the quilc server.
* ``QCS_SETTINGS_APPLICATIONS_QVM_URL``: URL for the QVM server.
* ``QCS_SETTINGS_APPLICATIONS_API_URL``: URL for the QCS REST API server.
* ``QCS_SETTINGS_APPLICATIONS_GRPC_URL``: URL for the QCS gRPC API.

See the `QCS Client Configuration Guide <https://docs.rigetti.com/qcs/references/qcs-client-configuration>`_ for more details.
"""
