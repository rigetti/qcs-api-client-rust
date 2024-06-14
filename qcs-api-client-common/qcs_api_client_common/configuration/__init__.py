"""This module is used for loading configuration that will be used to connect either to real QPUs (and supporting services) or the QVM.

By default, all settings are loaded from files located under your home directory in the
``.qcs`` folder. Within that folder:

* ``settings.toml`` will be used to load general settings (e.g. which URLs to connect to).
* ``secrets.toml`` will be used to load tokens for authentication.

Both files should contain profiles. Your settings should contain a ``default_profile_name``
that determines which profile is loaded when no other profile is explicitly specified.

If you don't have either of these files, see [the QCS credentials guide](https://docs.rigetti.com/qcs/guides/qcs-credentials) for details on how to obtain them.

You can use environment variables to override values in your configuration:

* ``QCS_SETTINGS_FILE_PATH``: Set the path of the ``settings.toml`` file to load.
* ``QCS_SECRETS_FILE_PATH``: Set the path of the ``secrets.toml`` file to load.
* ``QCS_PROFILE_NAME``: Set the profile to load from ``settings.toml``
* ``QCS_SETTINGS_APPLICATIONS_QUILC_URL``: Override the URL used for requests to the quilc server.
* ``QCS_SETTINGS_APPLICATIONS_QVM_URL``: Override the URL used for requests to the QVM server.
* ``QCS_SETTINGS_APPLICATIONS_API_URL``: Override the URL used for requests to the QCS REST API server.
* ``QCS_SETTINGS_APPLICATIONS_GRPC_URL``: Override the URL used for requests to the QCS gRPC API.

The ``ClientConfiguration`` exposes an API for loading and accessing your
configuration.
"""
