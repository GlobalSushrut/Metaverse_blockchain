# Security Policy

## Supported Versions

Currently being updated with security patches:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Quantum Metaverse seriously. If you believe you have found a security vulnerability, please report it to us as described below.

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to umeshlamton@gmail.com.

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the requested information listed below to help us better understand the nature and scope of the possible issue:

* Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
* Full paths of source file(s) related to the manifestation of the issue
* The location of the affected source code (tag/branch/commit or direct URL)
* Any special configuration required to reproduce the issue
* Step-by-step instructions to reproduce the issue
* Proof-of-concept or exploit code (if possible)
* Impact of the issue, including how an attacker might exploit it

## Quantum-Specific Security Considerations

Given the quantum nature of this project, we have additional security considerations:

1. **Post-Quantum Cryptography**
   - All cryptographic implementations must be quantum-resistant
   - Regular audits of cryptographic protocols

2. **Quantum State Security**
   - Protection against quantum state manipulation
   - Quantum entropy source validation

3. **Hybrid Classical-Quantum Security**
   - Secure classical-quantum interfaces
   - Protection against hybrid attack vectors

## Preferred Languages

We prefer all communications to be in English.

## Policy

1. The security team will acknowledge your email within 48 hours.
2. You will receive a more detailed response within 72 hours.
3. We will create a security advisory on GitHub to track the issue.
4. Once the issue is confirmed and fixed, we will release a security update.
5. We will credit you in the security advisory.
