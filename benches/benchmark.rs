use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use malgebra::vector::*;

// criterion_group!(benches, bench_mult_num_slice, bench_mult_with_other_self);
criterion_main!(benches);
