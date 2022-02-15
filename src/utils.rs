use std::f64::consts::PI;

const F32_ONE_BITS: u32 = 0x3F80_0000;
const F64_ONE_BITS: u64 = 0x3FF0_0000_0000_0000;

// Scales [old_min, old_max] to [new_min, new_max]
pub fn rescale(old_min: f64, old_max: f64, new_min: f64, new_max: f64, x: f64) -> f64 {
    let diff_ratio = (new_max - new_min) / (old_max - old_min);

    (x - old_min).mul_add(diff_ratio, new_min)
}

// Scales [0.0, 1.0] to [-1.0, 1.0]
pub fn unit_to_neg_unit(x: f64) -> f64 {
    x.mul_add(2.0, -1.0)
}

// Scales [-1.0, 1.0] to [0.0, 1.0]
pub fn neg_unit_to_unit(x: f64) -> f64 {
    x.mul_add(0.5, 0.5)
}

// Linear interpolation
pub fn lerp(bias: f64, lhs: f64, rhs: f64) -> f64 {
    (rhs - lhs).mul_add(bias, lhs)
}

// Cosine interpolation
pub fn cerp(bias: f64, lhs: f64, rhs: f64) -> f64 {
    let bias = (1.0 - (bias * PI).cos()) / 2.0;

    (rhs - lhs).mul_add(bias, lhs)
}

// All f32/f64 values in the range [1.0, 2.0) differ only in the mantissa
pub fn f32_from_mantissa(mantissa: u32, min: f32, max: f32) -> f32 {
    let float = f32::from_bits(F32_ONE_BITS ^ (mantissa >> 9));

    float.mul_add(max - min, min * 2.0 - max)
}

pub fn f64_from_mantissa(mantissa: u64, min: f64, max: f64) -> f64 {
    let float = f64::from_bits(F64_ONE_BITS ^ (mantissa >> 12));

    float.mul_add(max - min, min * 2.0 - max)
}

// Returns 6x^5 - 15x^4 + 10x^3
// Maps [0.0, 1.0] -> [0.0, 1.0]
pub fn smoothstep(x: f64) -> f64 {
    let a = x.mul_add(6.0, -15.0);
    let b = x.mul_add(a, 10.0);

    x.powi(3) * b
}

// Extension of `smoothstep` that works with negative numbers
// Maps [-1.0, 1.0] -> [-1.0, 1.0]
pub fn neg_smoothstep(x: f64) -> f64 {
    let x = x.mul_add(0.5, 0.5);

    let a = x.mul_add(12.0, -30.0);
    let b = x.mul_add(a, 20.0);

    x.powi(3).mul_add(b, -1.0)
}

// Maps [0.0, 1.0] -> [0.0, 1.0]
pub fn sigmoid(beta: f64, x: f64) -> f64 {
    1.0 / (1.0 + (x / (1.0 - x)).powf(beta))
}

// Extension of `sigmoid` that works with negative numbers
// Maps [-1.0, 1.0] -> [-1.0, 1.0]
pub fn neg_sigmoid(beta: f64, x: f64) -> f64 {
    (2.0 / (1.0 + ((x + 1.0) / (1.0 - x)).powf(beta))) - 1.0
}
