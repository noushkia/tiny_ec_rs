use crate::curve::Curve;
use crate::point::Point;
use num_bigint::BigInt;
use num_traits::Zero;

pub struct Keypair<'c> {
    pub curve: &'c Curve,
    private_key: BigInt,
    pub public_key: Point<'c>,
    pub signs: bool,
    pub encrypts: bool,
}

impl<'c> Keypair<'c> {
    pub fn new(
        curve: &'c Curve,
        private_key: Option<&BigInt>,
        public_key: Option<&Point<'c>>,
    ) -> Option<Self> {
        if private_key.is_none() && public_key.is_none() {
            eprintln!("Error: No private key or public key provided");
            return None;
        }
        let g = Point::new(curve, curve.field.g.0.clone(), curve.field.g.1.clone())
            .expect("Error creating curve point for g");
        let signs = !private_key.is_none();
        let private_key = if private_key.is_none() {
            BigInt::zero()
        } else {
            private_key.unwrap().clone()
        };
        let public_key = if public_key.is_none() {
            Point::mul_double_and_add(&g, private_key.clone()).unwrap()
        } else {
            public_key.unwrap().clone()
        };
        Some(Keypair {
            curve,
            private_key,
            public_key,
            signs,
            encrypts: false,
        })
    }
}
