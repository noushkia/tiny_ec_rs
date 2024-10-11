use crate::curve::Curve;
use num_bigint::BigInt;

pub struct Point<'a> {
    curve: &'a Curve,
    x: BigInt,
    y: BigInt,
    on_curve: bool,
}

impl<'a> Point<'a> {
    pub fn new(curve: &'a Curve, x: BigInt, y: BigInt) -> Self {
        let on_curve = curve.on_curve(&x, &y);
        if !on_curve {
            eprintln!("Point ({}, {}) is not on curve \"{}\"", x, y, curve.name);
        }
        Point {
            curve,
            x,
            y,
            on_curve,
        }
    }

    fn add(&self, other: &Point) -> Point {
        if self.curve != other.curve {
            panic!("Cannot add points belonging to different curves");
        }

        if self.x == other.x && self.y != other.y {
            return Point::new(self.curve, BigInt::from(0), BigInt::from(0)); // Point at infinity
        }

        let m: BigInt = if self.x == other.x {
            (3 * &self.x.pow(2) + &self.curve.a) * (&BigInt::from(2) * &self.y).modinv(&self.curve.field.p).expect("Error computing modinv for 2y % p")
        } else {
            (&self.y - &other.y) * (&self.x - &other.x).modinv(&self.curve.field.p).expect("Error computing modinv for x1 - x2 % p")
        };

        let x_r = (m.modpow(&BigInt::from(2), &self.curve.field.p) - &self.x - &other.x) % &self.curve.field.p;
        let y_r = -(&self.y + m * (&x_r - &self.x)) % &self.curve.field.p;

        Point::new(self.curve, x_r, y_r)
    }
}