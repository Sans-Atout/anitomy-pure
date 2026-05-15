# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| latest  | Yes       |
| < latest | No       |

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security vulnerabilities.

Send a report to **augustin@rousset-rouviere.fr** with:

- A description of the vulnerability
- Steps to reproduce (a filename string or minimal code snippet is usually enough)
- The potential impact

You will receive a response within 7 days. If the issue is confirmed, a fix will be released as soon as possible and you will be credited in the changelog unless you prefer to remain anonymous.

## Scope

anitomy-pure is a filename parser. It takes a `&str` as input and returns structured data. It does not perform I/O, execute subprocesses, or communicate over a network. The attack surface is therefore limited to:

- **Memory safety**: the library is pure safe Rust with zero `unsafe` blocks
- **Denial of service via crafted input**: if you find an input that causes excessive CPU or memory use, please report it
