use crate::curve::Curve;
use crate::point::Point;
use num_bigint::{BigInt, RandBigInt};

pub struct Keypair<'a> {
    curve: &'a Curve,
    private_key: BigInt,
    public_key: Point<'a>,
}

impl<'a> Keypair<'a> {
    fn new(curve: &'a Curve) -> Self {
        let mut rng = rand::thread_rng();
        let priv_key = rng.gen_bigint_range(&BigInt::from(1), &curve.field.n);
        let pub_key = Point::new(&curve, curve.field.clone().g.0, curve.field.clone().g.1).expect("Error creating public key!");
        Keypair {
            curve,
            private_key: priv_key,
            public_key: pub_key,
        }
    }
}
