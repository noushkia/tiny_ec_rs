// use num_bigint::BigInt;
//
// pub fn egcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
//     if a == 0 {
//         (b, 0, 1)
//     } else {
//         let (g, y, x) = egcd(b % a, a);
//         (g, x - (b / a) * y, y)
//     }
// }
//
// pub fn mod_inv(a: BigInt, p: BigInt) -> i64 {
//     let (g, x, _) = egcd(a, p);
//     if g != 1 {
//         panic!("Modular inverse does not exist");
//     } else {
//         (x % p + p) % p
//     }
// }
