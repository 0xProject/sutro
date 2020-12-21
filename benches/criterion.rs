use criterion::Criterion;
use sutro::bench;

fn main() {
    let mut criterion = Criterion::default().configure_from_args();
    bench::main(&mut criterion);
    criterion.final_summary();
}
