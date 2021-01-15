//! # domain
//! the `domain` module extracts some struct and interfaces (Traits) in order to avoid cyclic
//! depednencies between apps.
//! As the submodules depends of the `kernel::Error` type, in order to extract the module from the kernel
//! we need to find a way to avoid cyclic dependencies between the kernel, the domain and the error types.
pub mod analytics;
pub mod files;
pub mod inbox;
pub mod messages;
