# BLAKE3 C Wrapper

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance C-compatible shared library for the BLAKE3 cryptographic hash function, built in Rust.

This wrapper is designed to be a simple, fast, and safe way to use BLAKE3 from any C-compatible language, with a primary focus on providing a performance boost for PHP extensions.

## Features

-   **Fast**: Leverages the official, highly-optimized Rust implementation of BLAKE3, including SIMD and multi-threading capabilities.
-   **Safe**: Built with Rust's memory safety guarantees.
-   **C-Compatible**: Exposes a clean C Application Binary Interface (ABI), allowing it to be loaded as a shared library (`.so`, `.dll`, `.dylib`).

## Build

You will need the Rust toolchain installed.

1.  Clone the repository.
2.  Navigate to the `blake3-wrapper` directory.
3.  Build the release version of the library:
    ```bash
    cargo build --release
    ```
4.  The compiled shared library will be available in the `target/release/` directory.
    -   On Linux: `libblake3_wrapper.so`
    -   On macOS: `libblake3_wrapper.dylib`
    -   On Windows: `blake3_wrapper.dll`

## Usage

You can link this shared library in your C/C++ project or load it dynamically. You will need a corresponding header file (`blake3_wrapper.h`) to declare the functions you expose from `lib.rs`.

**Example C code:**

```c
// (Assuming you have a blake3_wrapper.h header file)
#include "blake3_wrapper.h"
#include <stdio.h>

int main() {
    const char* input = "hello world";
    unsigned char output[BLAKE3_OUT_LEN];

    // Call the function from the Rust library
    blake3_hash(input, output);

    printf("BLAKE3 Hash: ");
    for(size_t i = 0; i < BLAKE3_OUT_LEN; ++i) {
        printf("%02x", output[i]);
    }
    printf("\n");

    return 0;
}
```

## More Information

To see our other projects and learn more about our commitment to open source, please visit our website.

-   **Our Open Source Contributions**: [https://codernix.com/open-source](https://codernix.com/open-source)
-   **BLAKE3 Wrapper Project Page**: [https://codernix.com/open-source/encryption/blake3](https://codernix.com/open-source/encryption/blake3)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.