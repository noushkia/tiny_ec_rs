use crate::curve::Curve;
use crate::utils::modsqrt;
use num_bigint::BigInt;
use num_traits::{Euclid, One, ToPrimitive, Zero};
use std::fmt;

#[derive(Clone)]
pub struct Point<'c> {
    curve: &'c Curve,
    pub x: BigInt,
    pub y: BigInt,
}

impl<'c> Point<'c> {
    pub fn new(curve: &'c Curve, x: BigInt, y: BigInt) -> Option<Self> {
        let on_curve = curve.on_curve(&x, &y);
        if !on_curve {
            eprintln!("Point ({}, {}) is not on curve \"{}\"", x, y, curve.name);
            return None;
        }
        Some(Point { x, y, curve })
    }

    #[inline]
    pub fn inf(curve: &'c Curve) -> Point<'c> {
        Point {
            curve,
            x: BigInt::zero(),
            y: BigInt::zero(),
        }
    }

    pub fn add(this: &Point<'c>, other: &Point<'c>) -> Option<Point<'c>> {
        if this.curve != other.curve {
            eprintln!("Cannot add points belonging to different curves");
            return None;
        }

        if this == &Point::inf(&this.curve) {
            return Some(other.clone());
        } else if other == &Point::inf(&other.curve) {
            return Some(this.clone());
        }

        if this.x == other.x && (&other.y + &this.y) % &other.curve.field.p == BigInt::zero() {
            return Some(Point::inf(&this.curve));
        }

        let m: BigInt = if this.x == other.x {
            ((3 * &this.x.pow(2) + &this.curve.a)
                * (&BigInt::from(2) * &this.y)
                    .modinv(&this.curve.field.p)
                    .expect("Error computing modinv for 2y % p"))
                % &this.curve.field.p
        } else {
            ((&other.y - &this.y)
                * (&other.x - &this.x)
                    .modinv(&this.curve.field.p)
                    .expect("Error computing modinv for x1 - x2 % p"))
                % &this.curve.field.p
        };

        let x_r = (m.modpow(&BigInt::from(2), &this.curve.field.p) - &this.x - &other.x)
            .rem_euclid(&this.curve.field.p);
        let y_r = (m * (&this.x - &x_r) - &this.y).rem_euclid(&this.curve.field.p);

        Point::new(&this.curve, x_r, y_r)
    }

    pub fn mul_double_and_add(p: &Point<'c>, mut n: BigInt) -> Option<Point<'c>> {
        let mut q = p.clone();
        let mut r = Point::inf(p.curve);
        while n > BigInt::zero() {
            if &n % BigInt::from(2) == BigInt::one() {
                r = Point::add(&r, &q).expect("Error adding points!");
            }
            q = Point::add(&q, &q).expect("Error doubling point q!");
            n /= 2;
        }
        Some(r)
    }

    // n: number of binary bits in k
    pub fn mul_montgomery(p: &Point<'c>, k: BigInt, n: usize) -> Option<Point<'c>> {
        let k_bin = k.to_radix_le(2).1;
        if k_bin[0] == 0 {
            eprintln!("k's msb must be 1!");
            return None;
        }
        let mut r0 = p.clone();
        let mut r1 = Self::add(p, p).expect("Error doubling P!");
        for i in (0..n - 1).rev() {
            if k_bin[i] == 0 {
                r1 = Self::add(&r0, &r1).expect("Error adding R0 and R1!");
                r0 = Self::add(&r0, &r0).expect("Error doubling R0!");
            } else {
                r0 = Self::add(&r0, &r1).expect("Error adding R0 and R1!");
                r1 = Self::add(&r1, &r1).expect("Error doubling R1!");
            }
        }
        Some(r0)
    }

    pub fn compress(&self) -> (BigInt, u8) {
        (
            self.x.clone(),
            (self.y.clone() % BigInt::from(2))
                .to_u8()
                .expect("Error checking the parity"),
        )
    }

    pub fn decompress(curve: &'c Curve, x: BigInt, is_odd: u8) -> Point<'c> {
        let sqrt = modsqrt(&(&x.pow(3) + &curve.a * &x + &curve.b), &curve.field.p)
            .expect("Error calculating the modular s");
        let y = if is_odd == 1 {
            sqrt
        } else {
            &curve.field.p - sqrt
        };
        Point::new(curve, x, y).unwrap()
    }
}

impl<'c> PartialEq for Point<'c> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.curve == other.curve
    }
}

impl<'c> Eq for Point<'c> {}

impl<'c> fmt::Debug for Point<'c> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl<'c> fmt::Display for Point<'c> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
