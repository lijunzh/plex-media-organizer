use criterion::{Criterion, black_box, criterion_group, criterion_main};
use plex_media_organizer::MovieParser;

fn simple_benchmark(c: &mut Criterion) {
    let parser = MovieParser::new(None);

    c.bench_function("parse_simple_filename", |b| {
        b.iter(|| {
            black_box(
                parser
                    .parse_filename("The.Matrix.1999.1080p.BluRay.mkv")
                    .unwrap(),
            );
        });
    });
}

criterion_group!(benches, simple_benchmark);
criterion_main!(benches);
