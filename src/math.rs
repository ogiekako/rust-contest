//! math library.

/// ```
/// use contest::math;
/// assert_eq!(3, math::inv(2,5));
/// assert_eq!(1, math::inv(1,2));
/// assert_eq!(1, math::inv(3,2));
/// ```
pub fn inv(value: i32, modulo_prime: i32) -> i32 {
    return pow_mod(value, modulo_prime - 2, modulo_prime);
}

/// ```
/// use contest::math;
/// assert_eq!(8, math::pow_mod(2,3,10));
/// assert_eq!(1, math::pow_mod(3,4,10));
/// assert_eq!(1, math::pow_mod(0,0,3));
/// assert_eq!(0, math::pow_mod(0,0,1));
/// ```
pub fn pow_mod(value: i32, exponent: i32, modulo: i32) -> i32 {
    if exponent < 0 {
        panic!("Exponent should >= 0.");
    }
    if modulo == 1 {
        return 0;
    }
    let modulo = modulo as i64;
    let mut value = value as i64;
    if value >= modulo {
        value %= modulo;
    }
    if value < 0 {
        value = value % modulo + value;
    }
    let mut res = 1;
    let mut exponent = exponent;
    while exponent > 0 {
        if (exponent & 1) == 1 {
            res = res * value % modulo;
        }
        value = value * value % modulo;
        exponent >>= 1;
    }
    res as i32
}
