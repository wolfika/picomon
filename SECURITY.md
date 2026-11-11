# Security policy

## Supported versions

Picomon is below 1.0. Security fixes go into the latest release only.

## Reporting a vulnerability

Report vulnerabilities privately, not in public issues:

- Use [Report a vulnerability](https://github.com/wolfika/picomon/security/advisories/new) on the repository's Security tab, or
- email info@wolfika.eu.

Include the Picomon version (`picomon --version`), how Picomon is deployed, and steps to reproduce.

Reports are acknowledged within 7 days. Once a fix is released, the issue is published as a GitHub security advisory, crediting the reporter unless they ask otherwise.

## Scope

In scope, for example:

- Bypassing basic auth when it is configured
- Reading files or data beyond the metrics Picomon serves
- Script or style injection into the dashboard
- Crashing or exhausting Picomon with requests from a client that has access to it

Out of scope, because the README documents them as deployment choices:

- Access to the dashboard when Picomon listens on an untrusted network without auth
- Credentials readable on the network because Picomon is served over plain HTTP; basic auth needs TLS from a reverse proxy
