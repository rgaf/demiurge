use std::hash::Hasher;
use std::ops::{BitXor, Shr};
use twox_hash::XxHash64;

const PRIME_1: u64 = 0x9E37_79B1_85EB_CA87;
const PRIME_2: u64 = 0xC2B2_AE3D_27D4_EB4F;
const PRIME_3: u64 = 0x1656_67B1_9E37_79F9;
const PRIME_4: u64 = 0x85EB_CA77_C2B2_AE63;
const PRIME_5: u64 = 0x27D4_EB2F_1656_67C5;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct StatelessRand {
    seed: u64
}

impl StatelessRand {
    pub fn from_seed(seed: u64) -> Self {
        Self { seed }
    }

    pub fn hash_bytes(&self, bytes: &[u8]) -> u64 {
        let mut hasher = XxHash64::with_seed(self.seed);

        hasher.write(&bytes);
        hasher.finish()
    }

    pub fn hash_1u64(&self, x: u64) -> u64 {
        let mut digest = self.prepare_seed(8);

        digest = StatelessRand::mix_u64(digest, x);

        StatelessRand::finalize(digest)
    }

    pub fn hash_2u64(&self, x: u64, y: u64) -> u64 {
        let mut digest = self.prepare_seed(16);

        digest = StatelessRand::mix_u64(digest, x);
        digest = StatelessRand::mix_u64(digest, y);

        StatelessRand::finalize(digest)
    }

    pub fn hash_3u64(&self, x: u64, y: u64, z: u64) -> u64 {
        let mut digest = self.prepare_seed(24);

        digest = StatelessRand::mix_u64(digest, x);
        digest = StatelessRand::mix_u64(digest, y);
        digest = StatelessRand::mix_u64(digest, z);

        StatelessRand::finalize(digest)
    }

    fn prepare_seed(&self, num_bytes: u64) -> u64 {
        self.seed.wrapping_add(PRIME_5).wrapping_add(num_bytes)
    }

    #[allow(dead_code)]
    fn mix_u8(lhs: u64, rhs: u8) -> u64 {
        let rhs = u64::from(rhs).wrapping_mul(PRIME_5);

        lhs.bitxor(rhs).rotate_left(11).wrapping_mul(PRIME_1)
    }

    #[allow(dead_code)]
    fn mix_u32(lhs: u64, rhs: u32) -> u64 {
        let rhs = u64::from(rhs).wrapping_mul(PRIME_1);

        lhs.bitxor(rhs).rotate_left(23).wrapping_mul(PRIME_2).wrapping_add(PRIME_3)
    }

    fn mix_u64(lhs: u64, rhs: u64) -> u64 {
        let rhs = rhs.wrapping_mul(PRIME_2).rotate_left(31).wrapping_mul(PRIME_1);

        lhs.bitxor(rhs).rotate_left(27).wrapping_mul(PRIME_1).wrapping_add(PRIME_4)
    }

    fn finalize(mut digest: u64) -> u64 {
        digest = digest.bitxor(digest.shr(33));
        digest = digest.wrapping_mul(PRIME_2);
        digest = digest.bitxor(digest.shr(29));
        digest = digest.wrapping_mul(PRIME_3);
        digest = digest.bitxor(digest.shr(32));

        digest
    }
}

#[cfg(test)]
mod test {
    use super::StatelessRand;

    #[test]
    fn small_inputs() {
        let rand = StatelessRand::from_seed(1234);

        let bytes: [u8; 24] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];

        assert_eq!(rand.hash_bytes(&bytes), rand.hash_3u64(0, 1, 2));
    }
}
