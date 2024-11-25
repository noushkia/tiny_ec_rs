#[cfg(test)]
mod tests {
    use tiny_ec::curve_registry::get_curve;

    #[test]
    fn test_invalid_curve_name() {
        let result = get_curve("abcd");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_curve_name() {
        let curve_name = "brainpoolP160r1";
        let result = get_curve(curve_name);

        assert!(result.is_ok());

        let curve = result.unwrap();
        assert_eq!(curve_name, curve.name);
    }
}
