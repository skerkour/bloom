# ct-logs
This is a crate containing Google's
[list of known Certificate Transparency logs](https://www.certificate-transparency.org/known-logs)
for use with the [sct.rs](https://github.com/ctz/sct.rs) crate.

[![Build Status](https://img.shields.io/travis/ctz/ct-logs.svg)](https://travis-ci.org/ctz/ct-logs)
[![Crate](https://img.shields.io/crates/v/ct-logs.svg)](https://crates.io/crates/ct-logs)

# License
Apache-2.0/ISC/MIT

# Regenerating sources
You will need python3 and curl.

Run `build.py` which will output a new version of `src/lib.rs`.  You can now
compare and audit.  The code is generated in deterministic order so changes
to the source should only result from upstream changes.

`build.py` also verifies the signature published alongside the list.
