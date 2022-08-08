# Rust OpenAPI Client Generator

This project generates two Rust clients—one for each OpenAPI schema (public and internal). New clients are generated automatically when the [qcs-api-docs] repo is updated.

To generate manually:

```
make regenerate
```

## The Public Code

The source code for the public version of the client is available [on GitHub](https://github.com/rigetti/qcs-api-client-rust) for the convenience of consumers and to give a place for users to open issues. **No commits should ever be made against GitHub** as they will be overwritten by this repository on release during CI. The process of updating the public code looks like this:

```mermaid
graph TD
service([A Service])
qcs_api_docs([qcs-api-docs])
service --New Partial Schema--> qcs_api_docs

subgraph wrapper["This Repo"]
  qcs_api_docs --push new schema--> generation([Generate Code])
  --When on main-->knope([Knope])
  -->update_version([Update Versions])
end

subgraph gh["GitHub qcs-api-client-rust"]
  public_code[(Public Repo)]
  update_version--Sync code & create release-->public_code
  --On Release-->gha([GitHub Actions])
end

gha-->crates[(Crates.io)]

```

[qcs-api-docs]: https://gitlab.com/rigetti/qcs/utilities/qcs-api-docs