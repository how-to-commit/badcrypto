# crypto (dont use)  
General cryptographic library with zero dependencies other than Rust's stdlib
(and `rand`) for learning about cryptography and math. Planning to implement
any algorithm (including non-secure ones).

## feature support  
- [x] Blake2b  
- [x] X25519  

## Subprojects  
- [bignum](./src/math/biguint.rs).

## TODO:  
- [ ] FFC: RSA, Merkle-Hellman
- [ ] ECC: secp256k1, Dual_EC_DRBG
- [ ] symmetric: ChaCha20
- [ ] bignum: division is off-by-one sometimes  
- [ ] bignum: make faster...  
- [ ] bignum: make shr and shl constant-time. (currently: vartime wrt shift)  
- [ ] zero all buffers on drop - write_volatile should be useful here.  
