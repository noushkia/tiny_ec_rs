use num_bigint::BigInt;
use tiny_ec::curve::{Curve, SubGroup};
use tiny_ec::point::Point;

fn main() {
    let field = SubGroup {
        p: BigInt::from(9739),
        g: (BigInt::from(1), BigInt::from(2)),
        n: BigInt::from(5),
        h: BigInt::from(1),
    };
    let curve = Curve::new(BigInt::from(497), BigInt::from(1768), field, "cryptoHack".to_string());

    let x = Point::new(&curve, BigInt::from(5323), BigInt::from(5438)).unwrap();
    println!("{}", Point::mul_double_and_add(&x, BigInt::from(1337)).unwrap());

    let p = Point::new(&curve, BigInt::from(2339), BigInt::from(2213)).unwrap();
    println!("crypto{{{}}}", Point::mul_double_and_add(&p, BigInt::from(7863)).unwrap());
}