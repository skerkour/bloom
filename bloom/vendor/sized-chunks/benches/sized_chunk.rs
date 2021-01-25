// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

use sized_chunks::Chunk;

use refpool::{Pool, PoolDefault, PoolRef};

const SIZES: &[usize] = &[1024, 2048, 4096, 8192, 16384, 32768];

fn alloc<A: Default + PoolDefault, R: Default>(c: &mut Criterion, name: &str) {
    let mut group = c.benchmark_group(name);
    for size in SIZES {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::new("standard", size), size, |b, &size| {
            b.iter_batched_ref(
                || Vec::with_capacity(size),
                |vec| {
                    for _ in 0..size {
                        vec.push(R::default());
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("empty pool", size), size, |b, &size| {
            b.iter_batched_ref(
                || (Pool::<A>::new(size), Vec::with_capacity(size)),
                |&mut (ref pool, ref mut vec)| {
                    for _ in 0..size {
                        vec.push(PoolRef::default(pool));
                    }
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("full pool", size), size, |b, &size| {
            b.iter_batched_ref(
                || {
                    let pool = Pool::<A>::new(size);
                    pool.fill();
                    (pool, Vec::with_capacity(size))
                },
                |&mut (ref pool, ref mut vec)| {
                    for _ in 0..size {
                        vec.push(PoolRef::default(pool));
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn alloc_chunk_usize(c: &mut Criterion) {
    alloc::<Chunk<usize>, Rc<Chunk<usize>>>(c, "alloc/usize")
}

criterion_group!(sized_chunk, alloc_chunk_usize);
criterion_main!(sized_chunk);
