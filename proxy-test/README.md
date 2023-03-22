proxy-test
---

Validate that client HTTP, HTTPS, and SOCKS5 proxies work with sdk clients.

## Command `cargo make docker-test`
Launch a local `socks5` proxy with `docker config` and test that both openapi and gRPC requests successfully route though the proxy and connect to QCS as expected. This is not run during CI.

## Command `cargo make test`
Run both openapi and gRPC requests against an in-process `socks5` mock to validate that proxy configurations at least attempt to connect to a proxy, without testing the proxy itself. This runs during CI.
