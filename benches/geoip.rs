#[macro_use]
extern crate criterion;
extern crate std_csv;

use std::io::{BufReader, Cursor};

use criterion::Criterion;

use std_csv::Parser;

const BYTES: &'static [u8] = include_bytes!("../tests/geoip.csv");
fn bench_next(c: &mut Criterion) {
    // let f = File::open("./tests/geoip.csv").unwrap();
    let f = Cursor::new(BYTES);
    let f = BufReader::new(f);
    let mut parser = Parser::new(f);
    c.bench_function("geoip.csv", |b| b.iter(|| parser.next() ));
}

criterion_group!(benches, bench_next);
criterion_main!(benches);