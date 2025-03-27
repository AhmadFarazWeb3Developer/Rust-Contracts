fn main() {
    let mut x: f32 = f32::MAX; // Set to maximum possible f32 value
    x = x * 2.0; // Force overflow
    println!("x: {}", x);
}

// Rust does not have f16 (16-bit) or f128 (128-bit) floating-point types by default.
// By default, Rust uses f64 for floating-point literals unless explicitly specified as f32:
