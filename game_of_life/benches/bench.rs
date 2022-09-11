use std::array;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn count_naive(c: &mut Criterion) {
    c.bench_function("count_naive", |b| {
        let neighbors: [bool; 8] = array::from_fn(|_| rand::random::<bool>());

        b.iter(|| {
            let c = black_box(neighbors).into_iter().filter(|&x| x).count();
            if c == 3 {
                1
            } else {
                0
            }
        })
    });
}

fn count_byte(c: &mut Criterion) {
    c.bench_function("count_byte", |b| {
        let neighbors: [bool; 8] = array::from_fn(|_| rand::random::<bool>());
        let arr = black_box([0; 256]);

        b.iter(|| {
            arr[black_box(neighbors)
                .iter()
                .fold(0u16, |v, b| (v << 1) + (*b as u16)) as usize]
        })
    });
}

criterion_group! {
    name = count_neighbors;
    config = Criterion::default().sample_size(1000);
    targets = count_naive, count_byte
}

criterion_main!(count_neighbors);
