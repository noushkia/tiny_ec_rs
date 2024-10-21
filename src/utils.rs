use num_bigint::BigInt;
use num_traits::One;

pub fn modsqrt(square: &BigInt, modulus: &BigInt) -> Option<BigInt> {
    if modulus % BigInt::from(4) == BigInt::from(3) {
        return Some(square.modpow(&((modulus + BigInt::one()) / BigInt::from(4)), modulus));
    }
    None
}
