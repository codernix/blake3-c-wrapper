/**
 * @author  Abou-Bakr Seddik Ouahabi (aboubakr[AT]codernix.com)
 * @brief   A C wrapper for the official BLAKE3 Rust implementation.
 * @site    https://codernix.com/open-source/cryptography/blake3
 *
 * This module exports the high-performance BLAKE3 hashing functions
 * for use within a PHP C extension.
 */

use blake3;

// The #[no_mangle] attribute is wrapped in unsafe() as required by Rust 2024 edition.
// This tells the compiler not to change the name of this function, so it can be
// called from C code using the name "blake3_hash".
#[unsafe(no_mangle)]
pub extern "C" fn blake3_hash(input: *const u8, input_len: usize, output: *mut u8) {
    // This unsafe block is necessary because we are dereferencing raw C pointers,
    // which Rust cannot guarantee are valid.
    unsafe {
        // Convert the raw input pointer to a Rust slice.
        let input_slice = std::slice::from_raw_parts(input, input_len);

        // Create a new Blake3 hasher.
        let mut hasher = blake3::Hasher::new();
        hasher.update(input_slice);

        // Finalize the hash and write the result to the output pointer.
        let hash = hasher.finalize();
        let output_slice = std::slice::from_raw_parts_mut(output, 32);
        output_slice.copy_from_slice(hash.as_bytes());
    }
}
