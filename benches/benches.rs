use map_macro::map;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::time::Duration;

fn reapeatedly_allocate_map() {
  for _ in 0..500_000 {
    map!{
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
    };
  }
}

fn bench_map_allocation(c: &mut Criterion) {
  c.bench_function(
    "map allocation",
    |b| b.iter(|| black_box(reapeatedly_allocate_map())),
  );
}

criterion_group!(
  name = benches;
  config = Criterion::default()
    .measurement_time(Duration::new(70, 0))
    .without_plots();
  targets = bench_map_allocation
);

criterion_main!(benches);
