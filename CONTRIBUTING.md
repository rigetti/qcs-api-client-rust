# Contributing

## Secrets and Public Crates

The following crates are mirrored to GitHub and published to [crates.io](https://crates.io):

- `qcs-api-client-common`
- `qcs-api-client-grpc`
- `qcs-api-client-openapi/public`

As such, all changes to these crates are subject to the
[Open Source Contribution Policy](https://rigetti.atlassian.net/wiki/spaces/SWE/pages/2645327874/Open-Source+Contribution+Policy#Changes).

### Commit hygiene

We use semantic versioning with [angular style](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines) commit messages.
In particular, we have the following scopes:
- `grpc`: applies to both the internal and public grpc crates
- `grpc-public`: applies to only the public grpc crate
- `grpc-internal`: applies to only the private grpc crate
- `openapi`: applies to both the internal and public openapi crates
- `openapi-public`: applies to only the public openapi crate
- `openapi-internal`: applies to only the private openapi crate

The scope is important as `knope` uses it to determine which crate's version needs bumping. If you include no scope,
or the wrong scope, you might version bump the wrong crate.

## OpenAPI

### Regenerate Files on Template Changes

When making changes to the OpenAPI templates, be sure to run `make regenerate` so that the changes are also applied as
part of the commit.

This prevents issues with an unrelated change having an unexpectedly large diff due to also generating previously-merged
changes.

### `cargo check` and `clippy`

The templates should, as much as possible, generate clean Rust code. That means no warnings or errors should surface when
running `cargo check` or `cargo clippy` on the generated code (or a project depending on the generated code).

When possible, any warnings or errors should be resolved by editing the templates to generate better code. When that is
not possible, often due to problems with the Rust generator itself (i.e. the Java code), `#[allow(lint_name)]`
annotations are permitted _when accompanied by an explanatory comment_.

See [lib.mustache](./custom_templates/lib.mustache) for examples.
