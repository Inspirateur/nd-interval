use std::{path::Path, ops::Range};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nd_interval::NdInterval;
const STEP: f32 = 0.05;

fn cov_bench(c: &mut Criterion) {
    let imap: Vec<([Range<f32>; 4], String)> = Vec::from_csv(Path::new("benches/plants.csv")).unwrap();
    c.bench_function(&format!("vec-coverage-{}^{}", 1./STEP, 4), |b| b.iter(|| 
        black_box(imap.coverage(STEP))
    ));
}

criterion_group!(vec, cov_bench);
criterion_main!(vec);