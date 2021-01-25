// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bitmaps::Bits;
use core::iter;

use ::arbitrary::{size_hint, Arbitrary, Result, Unstructured};

use crate::{types::ChunkLength, Chunk, InlineArray, SparseChunk};

#[cfg(feature = "ringbuffer")]
use crate::RingBuffer;

fn empty<T: 'static>() -> Box<dyn Iterator<Item = T>> {
    Box::new(iter::empty())
}

fn shrink_collection<T: Clone, A: Arbitrary>(
    entries: impl Iterator<Item = T>,
    f: impl Fn(&T) -> Box<dyn Iterator<Item = A>>,
) -> Box<dyn Iterator<Item = Vec<A>>> {
    let entries: Vec<_> = entries.collect();
    if entries.is_empty() {
        return empty();
    }

    let mut shrinkers: Vec<Vec<_>> = vec![];
    let mut i = entries.len();
    loop {
        shrinkers.push(entries.iter().take(i).map(&f).collect());
        i /= 2;
        if i == 0 {
            break;
        }
    }
    Box::new(iter::once(Vec::new()).chain(iter::from_fn(move || loop {
        let mut shrinker = shrinkers.pop()?;
        let x: Option<Vec<A>> = shrinker.iter_mut().map(|s| s.next()).collect();
        if x.is_none() {
            continue;
        }
        shrinkers.push(shrinker);
        return x;
    })))
}

impl<A, N> Arbitrary for Chunk<A, N>
where
    A: Arbitrary,
    N: ChunkLength<A> + 'static,
{
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
        u.arbitrary_iter()?.take(Self::CAPACITY).collect()
    }

    fn arbitrary_take_rest(u: Unstructured<'_>) -> Result<Self> {
        u.arbitrary_take_rest_iter()?.take(Self::CAPACITY).collect()
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::recursion_guard(depth, |depth| {
            let (_, upper) = A::size_hint(depth);
            (0, upper.map(|upper| upper * Self::CAPACITY))
        })
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let collections = shrink_collection(self.iter(), |x| x.shrink());
        Box::new(collections.map(|entries| entries.into_iter().collect()))
    }
}

#[cfg(feature = "ringbuffer")]
impl<A, N> Arbitrary for RingBuffer<A, N>
where
    A: Arbitrary,
    N: ChunkLength<A> + 'static,
{
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
        u.arbitrary_iter()?.take(Self::CAPACITY).collect()
    }

    fn arbitrary_take_rest(u: Unstructured<'_>) -> Result<Self> {
        u.arbitrary_take_rest_iter()?.take(Self::CAPACITY).collect()
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::recursion_guard(depth, |depth| {
            let (_, upper) = A::size_hint(depth);
            (0, upper.map(|upper| upper * Self::CAPACITY))
        })
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let collections = shrink_collection(self.iter(), |x| x.shrink());
        Box::new(collections.map(|entries| entries.into_iter().collect()))
    }
}

impl<A, N> Arbitrary for SparseChunk<A, N>
where
    A: Clone,
    Option<A>: Arbitrary,
    N: ChunkLength<A> + Bits + 'static,
{
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
        u.arbitrary_iter()?.take(Self::CAPACITY).collect()
    }

    fn arbitrary_take_rest(u: Unstructured<'_>) -> Result<Self> {
        u.arbitrary_take_rest_iter()?.take(Self::CAPACITY).collect()
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::recursion_guard(depth, |depth| {
            let (_, upper) = Option::<A>::size_hint(depth);
            (0, upper.map(|upper| upper * Self::CAPACITY))
        })
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let collections = shrink_collection(self.clone().option_drain(), |x| x.shrink());
        Box::new(collections.map(|entries| entries.into_iter().collect()))
    }
}

impl<A, T> Arbitrary for InlineArray<A, T>
where
    A: Arbitrary,
    T: 'static,
{
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
        u.arbitrary_iter()?.take(Self::CAPACITY).collect()
    }

    fn arbitrary_take_rest(u: Unstructured<'_>) -> Result<Self> {
        u.arbitrary_take_rest_iter()?.take(Self::CAPACITY).collect()
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::recursion_guard(depth, |depth| {
            let (_, upper) = A::size_hint(depth);
            (0, upper.map(|upper| upper * Self::CAPACITY))
        })
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let collections = shrink_collection(self.iter(), |x| x.shrink());
        Box::new(collections.map(|entries| entries.into_iter().collect()))
    }
}
