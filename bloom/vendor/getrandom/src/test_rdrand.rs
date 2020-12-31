// We only test the RDRAND-based RNG source on supported architectures.
#![cfg(any(target_arch = "x86_64", target_arch = "x86"))]

#[path = "rdrand.rs"]
mod rdrand;
use rdrand::getrandom_inner as getrandom;
#[path = "test_common.rs"]
mod test_common;
