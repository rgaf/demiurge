use rand::{Rng, RngCore, SeedableRng};
use rand::distributions::Distribution;
use rand_chacha::ChaCha8Rng;

use crate::utils;

#[derive(Clone, Debug)]
pub struct StatefulRand {
    inner_rng: ChaCha8Rng
}

impl StatefulRand {
    pub fn from_seed(seed: u64) -> Self {
        Self { inner_rng: ChaCha8Rng::seed_from_u64(seed) }
    }

    pub fn set_word_pos(&mut self, word_offset: u128) {
        self.inner_rng.set_word_pos(word_offset)
    }

    pub fn set_stream(&mut self, stream: u64) {
        self.inner_rng.set_stream(stream)
    }

    pub fn sample<T, D: Distribution<T>>(&mut self, distribution: D) -> T {
        self.inner_rng.sample(distribution)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.inner_rng.next_u32()
    }

    pub fn next_u64(&mut self) -> u64 {
        self.inner_rng.next_u64()
    }

    // Range: [0, n)
    pub fn get_u32(&mut self, range: u32) -> u32 {
        let max_value = range - 1;
        let mask = 0xFFFF_FFFF >> (max_value | 1).leading_zeros();

        loop {
            let attempt = self.next_u32() & mask;

            if attempt <= max_value {
                break attempt;
            };
        }
    }

    // Range: [0, n)
    pub fn get_u64(&mut self, range: u64) -> u64 {
        let max_value = range - 1;
        let mask = 0xFFFF_FFFF_FFFF_FFFF >> (max_value | 1).leading_zeros();

        loop {
            let attempt = self.next_u64() & mask;

            if attempt <= max_value {
                break attempt;
            };
        }
    }

    // Range: [0.0, 1.0)
    pub fn get_f32(&mut self) -> f32 {
        utils::f32_from_mantissa(self.next_u32(), 0.0, 1.0)
    }

    // Range: [0.0, 1.0)
    pub fn get_f64(&mut self) -> f64 {
        utils::f64_from_mantissa(self.next_u64(), 0.0, 1.0)
    }

    // Randomly rounds a FP value up or down. A value of `N.F` will have a
    // (0.F / 1.0)% chance to round away from 0, and a ((1.0 - 0.F) / 1.0)%
    // chance of rounding toward 0
    pub fn round_f32(&mut self, value: f32) -> i32 {
        let trunc = value.trunc() as i32;
        let fract = value.fract().abs();

        if self.get_f32() < fract {
            if value.is_sign_negative() {
                trunc - 1
            } else {
                trunc + 1
            }
        } else {
            trunc
        }
    }

    pub fn round_f64(&mut self, value: f64) -> i32 {
        let trunc = value.trunc() as i32;
        let fract = value.fract().abs();

        if self.get_f64() < fract {
            if value.is_sign_negative() {
                trunc - 1
            } else {
                trunc + 1
            }
        } else {
            trunc
        }
    }

    pub fn coin_flip(&mut self) -> bool {
        self.next_u32() & 1 > 0
    }

    pub fn x_chance_in_y(&mut self, x: u32, y: u32) -> bool {
        self.get_u32(y) < x
    }

    pub fn roll(&mut self, num: u32, sides: u32) -> i32 {
        (0..num).fold(0, |acc, _| acc + self.get_u32(sides) + 1) as i32
    }
}
