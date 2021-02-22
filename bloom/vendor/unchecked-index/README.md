
# unchecked-index

Unchecked indexing through the regular index syntax.

Using a wrapper type that requires an `unsafe` block to create.

- crates.io: https://crates.io/crates/unchecked-index
- travis: https://travis-ci.org/bluss/unchecked-index

*Note:* All unchecked indexing here is actually “checked” with *debug
assertions* when they are enabled (they are off by default in release
builds). This is a feature! Debug checking does **not** make your code safe,
 but it helps finding bugs in `unsafe` code. Test your code responsibly.

## Example

```rust

use unchecked_index::unchecked_index;

/// unsafe because: trusts the permutation to be correct
unsafe fn apply_permutation<T>(perm: &mut [usize], v: &mut [T]) {
    debug_assert_eq!(perm.len(), v.len());
    
    // use unchecked (in reality, debug-checked) indexing throughout
    let mut perm = unchecked_index(perm);
    
    for i in 0..perm.len() {
        let mut current = i;
        while i != perm[current] {
            let next = perm[current];
            // move element from next to current
            v.swap(next, current);
            perm[current] = current;
            current = next;
        }
        perm[current] = current;
    }
}
```

## How to contribute:

- Fix a bug or implement a new thing
- Include tests for your new feature
- Make a pull request


## Recent Changes

- 0.2.2

  - The crate is now always `no_std`.

- 0.2.1

  - Improve the (debug) assertion messages; fix a typo and always include
    the relevant quantities (start, end, length)

- 0.2.0

  - Add support for unchecked indexing with ranges (“slicing”)
  - Add two free functions, `get_unchecked` and `get_unchecked_mut`

- 0.1.1

  - Add `Copy` impl (for shared slices)

- 0.1.0

  - Initial release
