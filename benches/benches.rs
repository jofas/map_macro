#![cfg(not(tarpaulin_include))]

use map_macro::{map, set, vec_no_clone};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const COUNT: usize = 250_000;

fn reapeat<T>(f: impl Fn() -> T, count: usize) {
    for _ in 0..count {
        f();
    }
}

#[allow(clippy::unit_arg)]
fn bench_map_allocation(c: &mut Criterion) {
    c.bench_function("map allocation", |b| {
        b.iter(|| {
            black_box(reapeat(
                || {
                    map! {
                        0 => "a",
                        1 => "b",
                        2 => "c",
                        3 => "d",
                        4 => "e",
                        5 => "f",
                        6 => "g",
                        7 => "h",
                        8 => "i",
                        9 => "j",
                        10 => "k",
                        11 => "l",
                        12 => "m",
                        13 => "n",
                        14 => "o",
                        15 => "p",
                        16 => "q",
                        17 => "r",
                        18 => "s",
                        19 => "t",
                        20 => "u",
                        21 => "v",
                        22 => "w",
                        23 => "x",
                        24 => "y",
                        25 => "z",
                    }
                },
                COUNT,
            ));
        });
    });
}

#[allow(clippy::unit_arg)]
fn bench_set_allocation(c: &mut Criterion) {
    c.bench_function("set allocation", |b| {
        b.iter(|| {
            black_box(reapeat(
                || {
                    set! {
                        "a",
                        "b",
                        "c",
                        "d",
                        "e",
                        "f",
                        "g",
                        "h",
                        "i",
                        "j",
                        "k",
                        "l",
                        "m",
                        "n",
                        "o",
                        "p",
                        "q",
                        "r",
                        "s",
                        "t",
                        "u",
                        "v",
                        "w",
                        "x",
                        "y",
                        "z",
                    }
                },
                COUNT,
            ));
        });
    });
}

#[allow(clippy::unit_arg)]
fn bench_vec_no_clone_allocation(c: &mut Criterion) {
    c.bench_function("vec_no_clone allocation", |b| {
        b.iter(|| {
            black_box(reapeat(
                || {
                    vec_no_clone![
                        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
                        "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
                    ]
                },
                COUNT,
            ));
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .noise_threshold(0.1)
        .without_plots();
    targets = bench_map_allocation, bench_set_allocation, bench_vec_no_clone_allocation
);

criterion_main!(benches);
