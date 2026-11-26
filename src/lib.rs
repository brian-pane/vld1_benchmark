use core::arch::aarch64::{
    uint8x16_t, uint32x4_t, uint32x4x4_t, vaddq_u8, vld1q_u32_x4, vreinterpretq_u8_u32,
    vreinterpretq_u32_u8, vst1q_u32_x4,
};

/// Partial implementation of Vigenère cipher, to test the use of `vld1q_u32_x4` and `vst1q_u32_x4`.
/// Note: Only supports keys of exactly 16 bytes and texts that are multiples of 64 bytes.
///
/// # Safety
/// * Caller must ensure that the system supports the Arm Neon instruction set.
#[target_feature(enable = "neon")]
pub fn vigenere_cipher(key: uint8x16_t, buffer: &mut [u8]) {
    let (chunks, _remainder) = buffer.as_chunks_mut::<64>();
    for chunk in chunks {
        // SAFETY: `vld1q_u32_x4` reads 64 bytes unaligned, and `chunk` is 64 bytes.
        let input = unsafe { vld1q_u32_x4(chunk.as_ptr().cast()) };
        let output = uint32x4x4_t(
            encode(key, input.0),
            encode(key, input.1),
            encode(key, input.2),
            encode(key, input.3),
        );
        // SAFETY: `vst1q_u32_x4` writes 64 bytes unaligned, and `chunk` is 64 bytes.
        unsafe { vst1q_u32_x4(chunk.as_mut_ptr().cast(), output) };
    }
}

#[target_feature(enable = "neon")]
fn encode(key: uint8x16_t, value: uint32x4_t) -> uint32x4_t {
    let value = vreinterpretq_u8_u32(value);
    let sum = vaddq_u8(key, value);
    vreinterpretq_u32_u8(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::arch::aarch64::vld1q_u8;
    use rand;
    use rand::Rng;

    #[test]
    fn test_encode() {
        let key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let mut before = [0u8; 1024];
        rand::rng().fill_bytes(&mut before);
        let mut after = before.clone();
        // SAFETY: `vld1q_u8` reads 16 bytes and `key` is 16 bytes. Assuming Neon support for this demo.
        unsafe { vigenere_cipher(vld1q_u8(key.as_ptr()), &mut after) };
        for (index, (byte_before, byte_after)) in before.iter().zip(after.iter()).enumerate() {
            assert_eq!(*byte_after, byte_before.wrapping_add(key[index % 16]));
        }
    }
}
