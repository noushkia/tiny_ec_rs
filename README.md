# Rust Tiny EC

This is a tiny implementation of [Elliptic Curves](https://en.wikipedia.org/wiki/Elliptic_curve) in Rust.

The code is based on tiny-ec python library [here](https://github.com/alexmgr/tinyec).

Note that this is an extremely simple implementation of elliptic curve operations in Weirstrass form. It's also a work
in progress, so I'd really appreciate any recommendations and fixes.

# Future Goals

- Elliptic Curve Diffie-Hellman key exchange (ECDH)
- Montgomery Form
- ElGamal encryption scheme
- Optimizations