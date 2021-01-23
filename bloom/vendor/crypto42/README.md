<p align="center">
  <h3 align="center">crypto42-rs</h3>
  <p align="center">Easy to use correctly and hard to misuse cryptographic library in Rust (using libsodium as backend)</p>
</p>


--------

1. [Primitives](#primitives)
2. [Libsodium](#libsodium)
3. [Contributing](#contributing)
4. [Licensing](#licensing)
5. [Sponsoring](#sponsoring)
6. [Security](#security)
7. [Fuzzing](#fuzzing)

--------

## Project status

This project is in maintenance mode: only bugs and security issues will be worked on.


------------------------------------------------------------------


`crypto42-rs` is cross-platform, secure, easy to use, and hard to misuse cryptographic library in Rust,
using [libsodium](https://github.com/jedisct1/libsodium) as backend.

This document presents the high level design of the library, you can find detailed code documentation here: https://bloom42.gitlab.io/libs/crypto42-rs/crypto42

The goal of `crypto42-rs` is to keep it's API surface as minial as possible and to implement the less ciphers
as possible.

Only safe to use/implement ciphers are available in `crypto42`.


## Primitives

- Authenticated Encryption with Associated Data (primitive: AEAD)
<!-- - Streaming Authenticated Encryption with Associated Data (primitive: -->
<!-- Streaming AEAD) -->
- One-way hash functions (primitive: HASH).
- Key Derivation Functions (primitive: KDF)
<!-- - *deterministic* authenticated encryption with associated data (primitive: -->
<!-- Deterministic Aead) -->
<!-- - message authentication codes (primitive: MAC), -->
- Public-key signatures (primitive: SIGN)
<!-- - hybrid encryption (primitives: HybridEncrypt and HybridDecrypt). -->

| Primitive          | Algorithms                            |
| ------------------ | ----------------------------------------------- |
| AEAD               | XCHACHA20_POLY1305 |
| Hash               | BLAKE2B, SHA2_512, SHA2_256, (SHA3_512, SHA3_256) |
| Key Derivation Functions | ARGON2ID, ARGON2I, SCRYPT |
| Public-key signatures | ED25519 |

<!-- | Streaming AEAD     | XCHACHA20_POLY1305 | -->
<!-- | Hybrid Encryption  | ECIES with AEAD and HKDF                        | -->
<!-- | MAC                | HMAC-SHA2                                       | -->
<!-- | Deterministic AEAD | AES-SIV | -->



## Libsodium

`crypto42-rs` uses `libsodium` as backend.

`libsodium` documentation: https://libsodium.gitbook.io


## Contributing

Thank you for your interest in contributing! Please refer to
https://gitlab.com/bloom42/wiki/-/wikis/organization/contributing for guidance.


## Licensing

`crypto42` is based on https://github.com/sodiumoxide/sodiumoxide but I don't remember how much ^^'

See `LICENSE.txt` and https://gitlab.com/bloom42/wiki/-/wikis/organization/licensing


## Security

If you found a security issue affecting this project, please do not open a public issue and refer to our
[dedicated security page](https://bloom.sh/security) instead. Thank you.


## Fuzzing

In the library's folder, use `make fuzz` to list fuzzing targets and then `cargo fuzz run [target]`
to run a specific target.
