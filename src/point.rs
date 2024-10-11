use std::fmt;
use crate::curve::Curve;
use num_bigint::BigInt;
use num_traits::{Euclid, Zero};

#[derive(Clone)]
pub struct Point<'a> {
    curve: &'a Curve,
    x: BigInt,
    y: BigInt
}

impl<'a> Point<'a> {

    pub fn new(curve: &'a Curve, x: BigInt, y: BigInt) -> Option<Self> {
        let on_curve = curve.on_curve(&x, &y);
        if !on_curve {
            eprintln!("Point ({}, {}) is not on curve \"{}\"", x, y, curve.name);
            return None;
        }
        Some(Point {
            x,
            y,
            curve: curve.clone()
        })
    }

    #[inline]
    pub fn inf(curve: &Curve) -> Point {
        Point {
            curve: curve.clone(),
            x: BigInt::from(0),
            y: BigInt::from(0)
        }
    }

    pub fn add(this: &Point<'a>, other: &Point<'a>) -> Option<Point<'a>> {
        if this.curve != other.curve {
            eprintln!("Cannot add points belonging to different curves");
            return None;
        }

        if this == &Point::inf(&this.curve) {
            return Some(other.clone());
        } else if other == &Point::inf(&other.curve) {
            return Some(this.clone());
        }

        if this.x == other.x &&  (&other.y + &this.y) % &other.curve.field.p == BigInt::zero() {
            return Some(Point::inf(&this.curve));
        }

        let m: BigInt = if this.x == other.x {
            ((3 * &this.x.pow(2) + &this.curve.a) * (&BigInt::from(2) * &this.y).modinv(&this.curve.field.p)
                .expect("Error computing modinv for 2y % p")
            ) % &this.curve.field.p
        } else {
            ((&other.y - &this.y) * (&other.x - &this.x).modinv(&this.curve.field.p)
                .expect("Error computing modinv for x1 - x2 % p")
            ) % &this.curve.field.p
        };

        let x_r = (m.modpow(&BigInt::from(2), &this.curve.field.p) - &this.x - &other.x).rem_euclid(&this.curve.field.p);
        let y_r = (m * (&this.x - &x_r) - &this.y).rem_euclid(&this.curve.field.p);

        Point::new(&this.curve, x_r, y_r)
    }
}

impl PartialEq for Point<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.curve == other.curve
    }
}

impl Eq for Point<'_> {}

impl fmt::Debug for Point<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}