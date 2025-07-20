#ifndef BLAKE3_WRAPPER_H
#define BLAKE3_WRAPPER_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief The length of a BLAKE3 hash in bytes.
 */
#define BLAKE3_OUT_LEN 32

/**
 * @brief   Computes the BLAKE3 hash of an input buffer.
 * @author  Abou-Bakr Seddik Ouahabi (aboubakr[AT]codernix.com)
 *
 * This function is an FFI wrapper around the official Rust BLAKE3 implementation.
 * It takes a pointer to an input buffer and writes the 32-byte hash into the
 * output buffer.
 *
 * @param[in]  input      A pointer to the input data.
 * @param[in]  input_len  The length of the input data in bytes.
 * @param[out] output     A pointer to a 32-byte buffer where the hash will be written.
 */
void blake3_hash(const uint8_t *input, size_t input_len, uint8_t *output);

#ifdef __cplusplus
}
#endif

#endif // BLAKE3_WRAPPER_H