# crypto (dont use)  
WIP cryptographic primitive library with zero dependencies other than Rust's 
stdlib (and `rand`) for learning about cryptography and math. 

## feature support  
- [x] Blake2b  
- [x] X25519  
- [ ] Poly1305  
- [ ] ChaCha20 (basically the same as BLAKE2?)

## more TODOs:  
- [ ] asymmetric ECC: secp256k1, Dual_EC_DRBG?  
- [ ] zero all buffers on drop - write_volatile should be useful here.  
- [ ] write more comprehensive tests
