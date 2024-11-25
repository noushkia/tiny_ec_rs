#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use tiny_ec::curve::{Curve, SubGroup};

    #[test]
    fn test_curve_is_singular() {
        let field = SubGroup {
            p: BigInt::from(23),
            g: (BigInt::from(1), BigInt::from(2)),
            n: BigInt::from(5),
            h: BigInt::from(1),
        };
        let curve1 = Curve {
            a: BigInt::from(0),
            b: BigInt::from(0),
            field: field.clone(),
            name: String::from("test_curve"),
        };
        assert!(curve1.is_singular());

        let curve2 = Curve {
            a: BigInt::from(-3),
            b: BigInt::from(2),
            field: field.clone(),
            name: String::from("test_curve"),
        };
        assert!(curve2.is_singular());

        let curve3 = Curve {
            a: BigInt::from(-3),
            b: BigInt::from(1),
            field: field.clone(),
            name: String::from("test_curve"),
        };
        assert!(!curve3.is_singular());

        let curve4 = Curve {
            a: BigInt::from(2),
            b: BigInt::from(2),
            field,
            name: String::from("test_curve"),
        };
        assert!(!curve4.is_singular());
    }
}

// #[cfg(test)]
// mod tests {
//     use tiny_ec::curve::{Curve, SubGroup};
//
//     #[test]
//     fn test_curve_is_singular() {
//         let field = SubGroup { p: 23, g: (1, 2), n: 5, h: 1 };
//
//         let singular_curve1 = Curve::new(0, 0, field.clone());
//         assert!(singular_curve1.is_singular());
//
//         let singular_curve2 = Curve::new(-3, 2, field.clone());
//         assert!(singular_curve2.is_singular());
//
//         let nonsingular_curve1 = Curve::new(-3, 1, field.clone());
//         assert!(!nonsingular_curve1.is_singular());
//
//         let nonsingular_curve2 = Curve::new(2, 2, field);
//         assert!(!nonsingular_curve2.is_singular());
//     }
// }
