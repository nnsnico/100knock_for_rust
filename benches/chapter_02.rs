use knock100::chapter_02;

use bencher::{benchmark_group, benchmark_main, Bencher};

fn sec_2_13(bench: &mut Bencher) {
    bench.iter(|| chapter_02::sec_2_13())
}

benchmark_group!(benches, sec_2_13_optimize);
benchmark_main!(benches);
