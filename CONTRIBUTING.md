# Contributing

## Secrets and Public Crates

The following crates are mirrored to GitHub and published to [crates.io](https://crates.io):

- `qcs-api-client-common`
- `qcs-api-client-grpc`
- `qcs-api-client-openapi/public`

As such, all changes to these crates are subject to the
[Open Source Contribution Policy](https://rigetti.atlassian.net/wiki/spaces/SWE/pages/2645327874/Open-Source+Contribution+Policy#Changes).

## OpenAPI

### Regenerate Files on Template Changes

When making changes to the OpenAPI templates, be sure to run `make regenerate` so that the changes are also applied as
part of the commit.

This prevents issues with an unrelated change having an unexpectedly large diff due to also generating previously-merged
changes.

### `cargo check` and `clippy`

The templates should, as much as possible, generate clean Rust code. That is, no warnings or errors should surface when
running `cargo check` or `cargo clippy` on the generated code (or a project depending on the generated code).

When possible, any warnings or errors should be resolved by editing the templates to generate better code. When that is
not possible, often due to problems with the Rust generator itself (i.e. the Java code), `#[allow(lint_name)]`
annotations are permitted _when accompanied by an explanatory comment_.

See [lib.mustache](./custom_templates/lib.mustache) for examples.
