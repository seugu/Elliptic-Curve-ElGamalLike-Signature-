# Elliptic-Curve-ElGamalLike-Signature

ElGamal-like signature on Secp256k1 Rust implementation that doesn't require any modular inverse. Based on the https://arxiv.org/ftp/arxiv/papers/1301/1301.2335.pdf proposal. Also, it contains generic EC implementation without the pairings. 

## Disclaimer

This is an **experimental** software and is provided on an "as is" and "as available" basis. We do **not give any warranties** and **will not be liable** for any losses incurred through any use of this code base.

## Test
Test for secp256k1 signing and verification
```
cargo test --package EC_ElGamalLike_Signature --bin EC_ElGamalLike_Signature -- test::test_secp256k1_sign_and_verify --exact --nocapture
```

Test for toy example from the Paper example 5.1. 
```
cargo test --package EC_ElGamalLike_Signature --bin EC_ElGamalLike_Signature -- test::test_sign --exact --nocapture 
```

## Documentation

EC ElGamalLike consists of three parts: keygen, signing, and verification. The upper-case letters are the EC Points and lower-case letters are the scalars.

### KeyGen 

- Choose the secret scalar **a** (PrivateKey)
- Compute the **B = aG** where G is the generator point (PublicKey)


### Signature 

- Choose the random scalars **k** and **l**
- Compute **R = kG**, and **S = lG**
- Compute a scalar **t = sk + rl + ma [mod q]** where s and r are the x-coordinates of S and R points, q is the order of the curve and m is the hash of a message represented in a scalar. 
- Signature is the tuple of **(R,S,t)**


### Verification

- Compute **tG**
- Compute **right_hand_side_point** = **sR + rS + mB**
- Check **right_hand_side_point** == **tG**
