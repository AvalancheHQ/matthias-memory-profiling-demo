use matthias_memory_profiling_demo::rle_encode;

#[divan::bench]
fn bench_rle_small_input() {
    let input = "aaabbbcccddd";
    divan::black_box(rle_encode(divan::black_box(input)));
}

#[divan::bench]
fn bench_rle_medium_input() {
    let input = "a".repeat(100) + &"b".repeat(100) + &"c".repeat(100);
    divan::black_box(rle_encode(divan::black_box(&input)));
}

#[divan::bench]
fn bench_rle_large_input() {
    let input = "a".repeat(10000) + &"b".repeat(10000) + &"c".repeat(10000);
    divan::black_box(rle_encode(divan::black_box(&input)));
}

#[divan::bench]
fn bench_rle_alternating_input() {
    let input = "abcdefghij".repeat(1000);
    divan::black_box(rle_encode(divan::black_box(&input)));
}

fn main() {
    divan::main();
}
