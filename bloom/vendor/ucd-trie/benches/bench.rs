#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;

use ucd_trie::TrieSetOwned;

#[bench]
fn bench_trie_set(b: &mut test::Bencher) {
    const CHARS: &'static [char] = &['a', 'β', '☃', '😼'];
    // const CHARS: &'static [char] = &['a'];
    lazy_static! {
        static ref SET: TrieSetOwned =
            TrieSetOwned::from_scalars(CHARS).unwrap();
    }

    let set = &*SET;
    let mut i = 0;
    b.iter(|| {
        let c = CHARS[i];
        i = (i + 1) % CHARS.len();

        for _ in 0..10000 {
            assert!(set.contains_char(c));
        }
    });
}
