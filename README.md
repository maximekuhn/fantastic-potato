# fantastic-potato

## reverse-proxy
- [x] full request loop (client -> reverse-proxy -> backend -> reverse-proxy -> client)
- [ ] config validation (yaml file)
- [ ] handle TLS/SSL
- [ ] caching
- [ ] load balancing
    - [x] random
    - [ ] round robin
    - [ ] least connection
    - [ ] ...
- [ ] healthcheck
- [ ] change config at runtime (validate before)
- [ ] get apps' backends dynamically (using a service discovery, ip ranges, ...)
- [ ] customise where logs are sent to (files, elasticsearch, ...)
- [ ] customise logs format (OpenTelemetry, ...)
- [ ] ...

## monitoring / web interface
- [ ] easy to understand metrics
- [ ] realtime logs
- [ ] change reverse-proxy's config
- [ ] get healthchecks status
- [ ] ...

