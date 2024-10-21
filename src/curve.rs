use num_bigint::BigInt;
use num_traits::Zero;
use std::fmt;

#[derive(Clone)]
// Simple Weierstrass curve structure
// y^2 = x^3 + a*x + b
pub struct Curve {
    pub a: BigInt,
    pub b: BigInt,
    pub field: SubGroup,
    pub name: String,
}

impl Curve {
    pub fn new(a: BigInt, b: BigInt, field: SubGroup, name: String) -> Self {
        Curve {
            a,
            b,
            field,
            name,
        }
    }

    fn mod_pow(base: &BigInt, exponent: usize, modulus: &BigInt) -> BigInt {
        base.modpow(&(BigInt::from(exponent)), &modulus)
    }

    pub fn is_singular(&self) -> bool {
        (4 * Self::mod_pow(&self.a, 3, &self.field.p) + 27 * Self::mod_pow(&self.b, 2, &self.field.p)) % &self.field.p == BigInt::zero()
    }

    pub fn on_curve(&self, x: &BigInt, y: &BigInt) -> bool {
        (Self::mod_pow(&y, 2, &self.field.p) - Self::mod_pow(&x, 3, &self.field.p) - &self.a * x - &self.b) % &self.field.p == BigInt::zero()
    }
}

impl fmt::Display for Curve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"{}\" => y^2 = x^3 + {}x + {} (mod {})",
            self.name, self.a, self.b, self.field.p
        )
    }
}

impl PartialEq for Curve {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.field == other.field
    }
}

impl Eq for Curve {}

#[derive(Clone)]
pub struct SubGroup {
    pub p: BigInt,              // Prime field of the subgroup curve points
    pub g: (BigInt, BigInt),    // Generator point coordinates
    pub n: BigInt,              // Order of the subgroup
    pub h: BigInt,              // Cofactor of the subgroup i.e. h = n / r where r is the order of the subgroup
}

impl fmt::Display for SubGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Subgroup => generator {:?}, order: {}, cofactor: {} on Field => prime {}",
            self.g, self.n, self.h, self.p
        )
    }
}

impl PartialEq for SubGroup {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.g == other.g && self.n == other.n && self.h == other.h
    }
}

impl Eq for SubGroup {}