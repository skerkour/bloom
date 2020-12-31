# GlobWalk #

[![Build Status](https://travis-ci.org/Gilnaa/globwalk.svg?branch=master)](https://travis-ci.org/Gilnaa/globwalk)
[![Build status](https://ci.appveyor.com/api/projects/status/81rkf5lcyt1ouh9n/branch/master?svg=true)](https://ci.appveyor.com/project/Gilnaa/globwalk)
[![](https://docs.rs/globwalk/badge.svg)](https://docs.rs/globwalk/)
![License](https://img.shields.io/crates/l/globwalk.svg)
[![crates.io](https://img.shields.io/crates/v/globwalk.svg)](https://crates.io/crates/globwalk)

Recursively find files in a directory using globs.

Based on both `walkdir` & `ignore` (❤), this crate inherits many goodies from
both, such as limiting search depth and amount of open file descriptors.

Licensed under MIT.

### Why not `glob` ###

 - The `glob` crate does not support having `{a,b}` in patterns.
 - `globwalk` can match several glob-patterns at the same time.
 - `globwalk` supports excluding results with `!`.
 - `glob` searches for files in the current working directory, whereas `globwalk` starts at a specified base-dir.

### Usage ###

To use this crate, add `globwalk` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
globwalk = "0.8.1"
```

The following piece of code recursively find all `png`, `jpg`, or `gif` files:

```rust
extern crate globwalk;

use std::fs;

for img in globwalk::glob("*.{png,jpg,gif}").unwrap() {
    if let Ok(img) = img {
        println!("{:?}", img.path());
    }
}
```

See the [documentation](https://docs.rs/globwalk/) for more details.
