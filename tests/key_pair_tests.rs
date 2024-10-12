#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use tiny_ec::key_pair::Keypair;
    use tiny_ec::point::Point;

    #[test]
    fn test_error_without_keys() {
        let curve = tiny_ec::curve_registry::get_curve("brainpoolP160r1").unwrap();

        let result = Keypair::new(&curve, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_public_key_calculation() {
        let curve = tiny_ec::curve_registry::get_curve("brainpoolP160r1").unwrap();
        let private_key = BigInt::from(1337);
        let keypair = Keypair::new(&curve, Some(&private_key), None).unwrap();
        let expected_pub_key = Point::mul_double_and_add(&Point::new(&curve, curve.field.g.0.clone(), curve.field.g.1.clone()).unwrap(), private_key).unwrap();
        assert_eq!(keypair.public_key, expected_pub_key);
    }
}