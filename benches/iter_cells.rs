use criterion::{black_box, criterion_group, criterion_main, Criterion};
use terminus::buffer::Buffer;

pub fn benchmark_iter_cells(c: &mut Criterion) {
    let buffer = Buffer::new(100, 100);

    c.bench_function("iter_cells with enumerate", |b| {
        b.iter(|| {
            for cell in black_box(&buffer).iter_cells() {
                black_box(cell);
            }
        })
    });

    c.bench_function("iter_cells without enumerate", |b| {
        b.iter(|| {
            let width = buffer.size.width;
            let mut index = 0;
            for cell in black_box(&buffer).cells.iter() {
                let x = (index % width as usize) as u16;
                let y = (index / width as usize) as u16;
                index += 1;
                black_box((x, y, cell));
            }
        })
    });
}

criterion_group!(benches, benchmark_iter_cells);
criterion_main!(benches);
