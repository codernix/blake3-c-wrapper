use blake3::Hasher;
use std::time::Instant;

fn main() {
    let input = b"Hello, World!";   // Example input
    let iterations = 1_000_000;     // 1 Million Number of iterations

    let mut hasher = Hasher::new();

    // Measure performance
    let start = Instant::now();
    for _ in 0..iterations {
        hasher.update(input);
        let _ = hasher.finalize();
    }
    let duration = start.elapsed();

    println!(
        "Hashed {} iterations in {:.2?} seconds.",
        iterations, duration
    );
}
