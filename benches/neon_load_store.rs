use std::arch::aarch64::vld1q_u8;
use criterion::{criterion_group, criterion_main, Criterion};

use vld1_benchmark::vigenere_cipher;

fn bench_load_store(c: &mut Criterion) {
    let mut buffer = [0u8; 1024];
    let key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    // SAFETY: `vld1q_u8` reads 16 bytes and `key` is 16 bytes. Assuming Neon support for this demo.
    let key = unsafe { vld1q_u8(key.as_ptr()) };

    c.bench_function("load_store", |b| {
        b.iter(|| {
            // SAFETY: Assuming Neon support for this demo.
            unsafe { vigenere_cipher(key, &mut buffer) };
        });
    });
}

criterion_group!(benches, bench_load_store);
criterion_main!(benches);
