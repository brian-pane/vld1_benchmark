This is a small benchmark program that uses the Rust `vld1q_u32_x4` and `vst1q_u32_x4` intrinsics to load and store four aarch64 Neon vector registers at a time.

The code does this, in a loop:
* Load the next 64 bytes from a buffer into registers using  `vld1q_u32_x4`.
* Compute a Vigenère cipher on that data, to simulate a real program doing some computation based on the loaded values.
* Write the cipher result back to the buffer using `vst1q_u32_x4`.

Limitations:
* This code will only run on aarch64 systems with the Neon instruction set. 

Usage:
```
# First, edit the rust-toolchain file to specify the rustc version to use.

cargo bench
```
