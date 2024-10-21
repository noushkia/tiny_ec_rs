use crate::curve::Curve;
use crate::point::Point;
use num_bigint::BigInt;

pub struct Keypair {
    pub curve: Curve,
    private_key: BigInt,
    pub public_key: Point,
    pub signs: bool,
    pub encrypts: bool,
}

impl<'a> Keypair {
    pub fn new(curve: &'a Curve, private_key: Option<&BigInt>, public_key: Option<&Point>) -> Option<Self> {
        if private_key.is_none() && public_key.is_none() {
            eprintln!("Error: No private key or public key provided");
            return None;
        }
        let g = Point::new(curve, curve.field.g.0.clone(), curve.field.g.1.clone()).unwrap();
        let signs = !private_key.is_none();
        let private_key = if private_key.is_none() { BigInt::from(0) } else { private_key.unwrap().clone() };
        let public_key = if public_key.is_none() { Point::mul_double_and_add(&g, private_key.clone()).unwrap() } else { public_key.unwrap().clone() };
        Some(Keypair {
            curve: curve.clone(),
            private_key,
            public_key,
            signs,
            encrypts: false,
        })
    }
}
