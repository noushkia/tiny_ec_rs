#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use tiny_ec::curve::{Curve, SubGroup};
    use tiny_ec::point::Point;

    #[test]
    fn test_addition_on_curve() {
        let field = SubGroup { p: BigInt::from(97), g: (BigInt::from(1), BigInt::from(2)), n: BigInt::from(5), h: BigInt::from(1) };
        let curve = Curve::new(BigInt::from(2), BigInt::from(3), field, "testCurve".to_string());

        let p1 = Point::new(&curve, BigInt::from(22), BigInt::from(5)).unwrap();
        let p2 = Point::new(&curve, BigInt::from(95), BigInt::from(31)).unwrap();
        let expected = Point::new(&curve, BigInt::from(29), BigInt::from(43)).unwrap();
        assert_eq!(expected, Point::add(&p1, &p2).unwrap());

        let p3 = Point::new(&curve, BigInt::from(24), BigInt::from(2)).unwrap();
        let p4 = Point::new(&curve, BigInt::from(96), BigInt::from(0)).unwrap();
        let expected2 = Point::new(&curve, BigInt::from(38), BigInt::from(90)).unwrap();
        assert_eq!(expected2, Point::add(&p3, &p4).unwrap());
    }

    #[test]
    fn test_warning_on_invalid_point() {
        let field = SubGroup {
            p: BigInt::from(97),
            g: (BigInt::from(1), BigInt::from(2)),
            n: BigInt::from(5),
            h: BigInt::from(1)
        };
        let curve = Curve::new(BigInt::from(2), BigInt::from(3), field, "testCurve".to_string());

        let p1 = Point::new(&curve, BigInt::from(22), BigInt::from(5)).unwrap();

        // Assuming your implementation raises a Result or Option for invalid points
        let result = Point::new(&curve, BigInt::from(94), BigInt::from(31));
        assert!(result.is_none());
    }

    #[test]
    fn test_addition_with_infinity() {
        let field = SubGroup {
            p: BigInt::from(97),
            g: (BigInt::from(1), BigInt::from(2)),
            n: BigInt::from(5),
            h: BigInt::from(1)
        };
        let curve = Curve::new(BigInt::from(2), BigInt::from(3), field, "testCurve".to_string());

        let p1 = Point::new(&curve, BigInt::from(22), BigInt::from(5)).unwrap();
        let inf = Point::inf(&curve);

        assert_eq!(p1, Point::add(&p1, &inf).unwrap());
        assert_eq!(p1, Point::add(&inf, &p1).unwrap());
    }

    #[test]
    fn test_doubling_on_curve() {
        let field = SubGroup {
            p: BigInt::from(97),
            g: (BigInt::from(1), BigInt::from(2)),
            n: BigInt::from(5),
            h: BigInt::from(1)
        };
        let curve = Curve::new(BigInt::from(2), BigInt::from(3), field, "testCurve".to_string());

        let p1 = Point::new(&curve, BigInt::from(24), BigInt::from(2)).unwrap();
        let expected = Point::new(&curve, BigInt::from(65), BigInt::from(65)).unwrap();
        assert_eq!(expected, Point::add(&p1, &p1).unwrap());
    }

    #[test]
    fn test_opposite_ordinates() {
        let field = SubGroup {
            p: BigInt::from(97),
            g: (BigInt::from(1), BigInt::from(2)),
            n: BigInt::from(5),
            h: BigInt::from(1)
        };
        let curve = Curve::new(BigInt::from(2), BigInt::from(3), field, "testCurve".to_string());

        let p1 = Point::new(&curve, BigInt::from(12), BigInt::from(3)).unwrap();
        let p2 = Point::new(&curve, BigInt::from(12), BigInt::from(94)).unwrap();
        let inf = Point::inf(&curve);
        assert_eq!(inf, Point::add(&p1, &p2).unwrap());
    }
}