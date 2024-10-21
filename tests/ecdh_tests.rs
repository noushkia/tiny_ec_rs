// #[cfg(test)]
// mod tests {
//     use tiny_ec::curve_registry::{ECDH, Keypair, Curve};
//
//     #[test]
//     fn test_secret_generation() {
//         let curve = your_crate::get_curve("secp384r1").unwrap();
//
//         let keypair1 = Keypair::generate(&curve);
//         let keypair2 = Keypair::generate(&curve);
//
//         let ecdh1 = ECDH::new(&keypair1);
//         let ecdh2 = ECDH::new(&keypair2);
//
//         let secret1 = ecdh1.get_secret(&keypair2.public);
//         let secret2 = ecdh2.get_secret(&keypair1.public);
//
//         assert_eq!(secret1, secret2);
//     }
// }