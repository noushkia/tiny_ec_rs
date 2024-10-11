use crate::curve::{Curve, SubGroup};
use num_bigint::BigInt;
use num_traits::Num;

pub struct CurveParams {
    pub p: BigInt,
    pub a: BigInt,
    pub b: BigInt,
    pub g: (BigInt, BigInt),
    pub n: BigInt,
    pub h: BigInt,
}

// https://github.com/alexmgr/tinyec/blob/master/tinyec/registry.py
pub fn get_curve(name: &str) -> Result<Curve, &'static str> {
    let curve_params = match name.to_lowercase().as_str() {
        "brainpoolp160r1" => CurveParams {
            p: BigInt::from_str_radix("e95e4a5f737059dc60dfc7ad95b3d8139515620f", 16).unwrap(),
            a: BigInt::from_str_radix("340e7be2a280eb74e2be61bada745d97e8f7c300", 16).unwrap(),
            b: BigInt::from_str_radix("1e589a8595423412134faa2dbdec95c8d8675e58", 16).unwrap(),
            g: (BigInt::from_str_radix("bed5af16ea3f6a4f62938c4631eb5af7bdbcdbc3", 16).unwrap(), BigInt::from_str_radix("1667cb477a1a8ec338f94741669c976316da6321", 16).unwrap()),
            n: BigInt::from_str_radix("e95e4a5f737059dc60df5991d45029409e60fc09", 16).unwrap(),
            h: BigInt::from_str_radix("1", 16).unwrap(),
        },
        "brainpoolp192r1" => CurveParams {
            p: BigInt::from_str_radix("c302f41d932a36cda7a3463093d18db78fce476de1a86297", 16).unwrap(),
            a: BigInt::from_str_radix("6a91174076b1e0e19c39c031fe8685c1cae040e5c69a28ef", 16).unwrap(),
            b: BigInt::from_str_radix("469a28ef7c28cca3dc721d044f4496bcca7ef4146fbf25c9", 16).unwrap(),
            g: (BigInt::from_str_radix("c0a0647eaab6a48753b033c56cb0f0900a2f5c4853375fd6", 16).unwrap(), BigInt::from_str_radix("14b690866abd5bb88b5f4828c1490002e6773fa2fa299b8f", 16).unwrap()),
            n: BigInt::from_str_radix("c302f41d932a36cda7a3462f9e9e916b5be8f1029ac4acc1", 16).unwrap(),
            h: BigInt::from_str_radix("1", 16).unwrap(),
        },
        _ => return Err("Unknown elliptic curve name"),
    };

    let sub_group = SubGroup {
        p: curve_params.p,
        g: curve_params.g,
        n: curve_params.n,
        h: curve_params.h,
    };

    let curve = Curve {
        a: curve_params.a,
        b: curve_params.b,
        field: sub_group,
        name: String::from(name),
    };

    Ok(curve)
}