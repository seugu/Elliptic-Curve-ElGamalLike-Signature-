use EC_ElGamalLike_Signature::FiniteField;
use EC_ElGamalLike_Signature::Point;
use EC_ElGamalLike_Signature::EllipticCurve;
use num_bigint::RandBigInt;
use num_bigint::{BigUint};


struct ElGamallikeSignature{
    ec: EllipticCurve,
    gen: Point,
    q: BigUint,
}

impl ElGamallikeSignature {
    pub fn generate_key_pair(&self) -> (BigUint,Point){
        let priv_key = self.generate_private_key(); 
        let pub_key = self.generate_pub_key(&priv_key);
        (priv_key,pub_key)
    }

    pub fn generate_private_key(&self) -> BigUint{
        self.generate_random_number_in_range(&self.q)
    }

    pub fn generate_pub_key(&self, priv_key: &BigUint) -> Point{
        self.ec.scalar_mul(&self.gen, &priv_key)
    }

    pub fn generate_random_number_in_range(&self, max:&BigUint) -> BigUint{
        let mut random_number_generator = rand::thread_rng();
        random_number_generator.gen_biguint_range(&BigUint::from(1u32), &max)
    }

    // SIGNING PART 
    // protocol => https://arxiv.org/ftp/arxiv/papers/1301/1301.2335.pdf 
    //signing process is as follows: 
    // choose an random k and l 
    // compute R = kG  and S = lG 
    // compute t = sk + rl + ma  mod q , where a is the private key and m is the message(hashed) and q is the order of the curve 
    // and s is the x coordinate of S 
    // r is the x coordinate of R 

    pub fn sign(
        &self,
        hash: &BigUint,
        priv_key: &BigUint,
        random_k: &BigUint,
        random_l: &BigUint
    ) -> (Point, Point, BigUint) {
        assert!( *hash < self.q, "hash cannot be bigger than of the EC group");
        assert!( *priv_key < self.q, "private key cannot be bigger than of the EC group");
        assert!( *random_k < self.q, "random k cannot be bigger than of the EC group");
        assert!( *random_l < self.q, "random k cannot be bigger than of the EC group");


        let r_point = self.ec.scalar_mul(&self.gen, random_k);
        let s_point = self.ec.scalar_mul(&self.gen, random_l);
        

        if let Point::Coor(r, _ ) = &r_point {
            if let Point::Coor(s,_ ) = &s_point {
                let sk = FiniteField::mul(&s, random_k, &self.q);
                let rl = FiniteField::mul(&r, random_l, &self.q);
                let mut t = FiniteField::add(&sk, &rl, &self.q);
                let ma = FiniteField::mul(hash, priv_key, &self.q);
                t = FiniteField::add(&t, &ma, &self.q);
                return (r_point, s_point, t);
            }
        }
        panic!("the random points cannot be the identity");


    // VERIFICATION PART 
    // protocol => https://arxiv.org/ftp/arxiv/papers/1301/1301.2335.pdf 
    //verification process is as follows: 
    // recall the signature (R,S,t) where R,S are points and t is scalar 
    // compute tG, sR, rS and mB where m is message(hash) and B is the public key
    // Signature is verified if tg == sR + rS + mB 
    } 

    pub fn verify(
        &self, 
        hash: &BigUint,
        pub_key: &Point,
        signature: &(Point, Point, BigUint)
    ) -> (bool) {
        assert!( *hash < self.q, "hash cannot be bigger than of the EC group");

        let (r_point,s_point,t) = signature;

        if let Point::Coor(r, _ ) = &r_point {
            if let Point::Coor(s, _) = &s_point  {
                let tg = self.ec.scalar_mul(&self.gen, &t);
                println!("tg = {:?}",tg);
                let sr = self.ec.scalar_mul(&r_point, &s);
                let rs = self.ec.scalar_mul(&s_point, &r);
                let mb = self.ec.scalar_mul(&pub_key, &hash);

                let mut sr_rs_mb = self.ec.add(&sr, &rs);
                sr_rs_mb = self.ec.add(&sr_rs_mb, &mb);
                println!("sR + rS + mB = {:?}\n",sr_rs_mb);
                if sr_rs_mb == tg {
                    return true;
                }
                return false;
            }
        }
    panic!("the signature points cannot be the identity");
}
}

fn main(){
    println!("try");
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_secp256k1_sign_and_verify(){
        // https://arxiv.org/ftp/arxiv/papers/1301/1301.2335.pdf example 5.1. 
        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 
            16
        ).unwrap();

        //order of the curve
        let q = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 
            16
        ).unwrap();


        //generator points of the field 
        let generator_x = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 
            16
        ).unwrap();

        let generator_y = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 
            16
        ).unwrap();


        //creating secp256k1 elliptic curve
        let ec = EllipticCurve{
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,
        };

        let gen = Point::Coor(generator_x, generator_y);


        let ElGamallikeSignature = ElGamallikeSignature{
            ec,
            gen,
            q,
        };

        let priv_key = ElGamallikeSignature.generate_private_key();
        println!("PrivateKey a = {:?}\n", priv_key);
        let pub_key = ElGamallikeSignature.generate_pub_key(&priv_key);
        println!("PubKey B = {:?}\n", pub_key);

        // creating a random hash; k_random and l_random we can use the same method creates private key
        let hash = ElGamallikeSignature.generate_private_key();
        let k_random = ElGamallikeSignature.generate_private_key();
        let l_random = ElGamallikeSignature.generate_private_key();



        let signature = ElGamallikeSignature.sign(&hash, &priv_key, &k_random, &l_random);
        println!("signature = {:?}\n", signature);

        let verify_result = ElGamallikeSignature.verify(&hash, &pub_key, &signature);
        assert!(verify_result, "verification should fail");


    }


    #[test]
    fn test_sign(){
        // https://arxiv.org/ftp/arxiv/papers/1301/1301.2335.pdf example 5.1. 
        let ec = EllipticCurve{
            a: BigUint::from(6u32),
            b: BigUint::from(2u32),
            p: BigUint::from(757u32),
        };

        let gen = Point::Coor(BigUint::from(529u32), BigUint::from(566u32));
        let q = BigUint::from(113u32);

        let ElGamallikeSignature = ElGamallikeSignature{
            ec,
            gen,
            q,
        };

        let priv_key = BigUint::from(78u32);
        let pub_key = ElGamallikeSignature.generate_pub_key(&priv_key);
        println!("PubKey B = {:?}", pub_key);

        let hash = BigUint::from(56u32);
        let k_random = BigUint::from(81u32);
        let l_random = BigUint::from(63u32);



        let signature = ElGamallikeSignature.sign(&hash, &priv_key, &k_random, &l_random);
        println!("{:?}", signature);


    }


    #[test]
    fn test_verify(){
        // https://arxiv.org/ftp/arxiv/papers/1301/1301.2335.pdf example 5.1. 
        let ec = EllipticCurve{
            a: BigUint::from(6u32),
            b: BigUint::from(2u32),
            p: BigUint::from(757u32),
        };

        let gen = Point::Coor(BigUint::from(529u32), BigUint::from(566u32));
        let q = BigUint::from(113u32);

        let ElGamallikeSignature = ElGamallikeSignature{
            ec,
            gen,
            q,
        };

        let priv_key = BigUint::from(78u32);
        let pub_key = ElGamallikeSignature.generate_pub_key(&priv_key);

        let hash = BigUint::from(56u32);
        let k_random = BigUint::from(81u32);
        let l_random = BigUint::from(63u32);



        let signature = ElGamallikeSignature.sign(&hash, &priv_key, &k_random, &l_random);
        println!("{:?}", signature);

        let verify_result = ElGamallikeSignature.verify(&hash, &pub_key, &signature);
        assert!(verify_result, "verification should fail");

    }
}