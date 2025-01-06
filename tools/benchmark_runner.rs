use std::time::Instant;
use std::fs::write;
use std::fs;

fn benchmark_encrypt(file_size_mb: usize, key: &[u8]) {
    let input_file = "benchmark_input.bin";
    let output_file = "benchmark_output.enc";

    // Generate a test file
    let data = vec![0xAA; file_size_mb * 1024 * 1024];
    write(input_file, &data).expect("Failed to create input file");

    let start = Instant::now();
    shellmorph_core::encrypt_file(input_file.into(), output_file.into(), key).unwrap();
    let duration = start.elapsed();

    println!(
        "Encryption of {} MB completed in {:.2?} seconds.",
        file_size_mb,
        duration
    );

    // Clean up
    fs::remove_file(input_file).unwrap();
    fs::remove_file(output_file).unwrap();
}

fn main() {
    let key = b"complexsecurekey";
    println!("Starting benchmark...");
    benchmark_encrypt(100, key);
}
