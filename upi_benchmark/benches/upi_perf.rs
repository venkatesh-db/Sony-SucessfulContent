use criterion::{black_box, criterion_group, criterion_main, Criterion};
use upi_benchmark::{create_upi_payload};

fn benchmark_upi_creation(c: &mut Criterion) {

    c.bench_function("create_upi_payload", |b| {
        b.iter(|| {
            let _ = create_upi_payload(
                black_box("venkat@upi"),
                black_box("merchant@upi"),
                black_box(500.0),
            );
        });
    });
}

criterion_group!(benches, benchmark_upi_creation);
criterion_main!(benches);
