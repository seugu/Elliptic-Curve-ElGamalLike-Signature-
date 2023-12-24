use num_bigint::{BigUint, BigInt};

/* struct Point{
    //we cannot use Point because sometimes we require identity
    x:BigInt,
    y:BigInt,
} */

#[derive(PartialEq, Clone, Debug)]
pub enum Point{
    Coor(BigUint,BigUint),
    Identity,
}
pub struct EllipticCurve{
    // y^2 = x^2 + ax + b
    pub a: BigUint,
    pub b: BigUint, 
    pub p: BigUint,
}

impl EllipticCurve {
    pub fn add(&self, c: &Point, d: &Point) -> Point{
        assert!(self.is_on_curve(c),"First point is not in curve");
        assert!(self.is_on_curve(d), "Second point is not in curve");
        assert!(c != d, "Points need to be different");

        match (c,d) {
            (Point::Identity,d) => d.clone(),
            (c, Point::Identity) => c.clone(),
            (Point::Coor(x1, y1), Point::Coor(x2,y2 )) => {
                let y1_plus_y2 = FiniteField::add(&y1, &y2, &self.p);
                if x1 == x2 && y1_plus_y2 == BigUint::from(0u32){
                    return Point::Identity;
                }
                // s = (y2 - y1) / (x2 - x1) mod p (slope of a line )
                // x3 = s^2 - x1 - x2 mod p 
                // y3 = s(x1 - x3) - y1 mod p 
                let y2_minus_y1 = FiniteField::subs(y2, y1, &self.p);
                let x2_minus_x1 = FiniteField::subs(x2, x1, &self.p);
                let s = FiniteField::div(&y2_minus_y1, &x2_minus_x1, &self.p);
                let x1_plus_x2 = FiniteField::add(x1, x2, &self.p);
                let s_square = FiniteField::mul(&s, &s, &self.p);
                let x3 = FiniteField::subs(&s_square, &x1_plus_x2, &self.p);
                let x1_minus_x3 = FiniteField::subs(x1, &x3, &self.p);
                let mut y3 = FiniteField::mul(&s, &x1_minus_x3, &self.p);
                y3 = FiniteField::subs(&y3,y1, &self.p);
                Point::Coor(x3, y3)
            } 
        }
      
    }

    pub fn double(&self, c: &Point) -> Point{
        assert!(self.is_on_curve(c),"First point is not in curve");
        match c {
            Point::Identity => Point::Identity,
            Point::Coor(x1, y1) => {
                if y1 == &BigUint::from(0u32) {
                    return Point::Identity;
                }
                //y^2 = x^2 + ax + b (derivative of the equation)
                // s = (3x1^2 + a) / (2 * y1) mod p 
                // x3 = s^2 - 2 * x1 mod p 
                // y3 = s(x1 - x3) - y1 mod p 
                let x1_square = FiniteField::mul(x1, x1, &self.p);
                let x1_square_3x = FiniteField::mul(&x1_square, &BigUint::from(3u32), &self.p);
                let x1_square_3x_plus_a = FiniteField::add(&x1_square_3x, &self.a, &self.p);
                let y1_2x = FiniteField::mul(y1, &BigUint::from(2u32),&self.p);
                let s = FiniteField::div(&x1_square_3x_plus_a, &y1_2x, &self.p);
                let x1_mul_2 = FiniteField::mul(x1, &BigUint::from(2u32), &self.p);
                let s_square = FiniteField::mul(&s, &s, &self.p);
                let x3 = FiniteField::subs(&s_square, &x1_mul_2, &self.p);
                let x1_minus_x3 = FiniteField::subs(x1, &x3, &self.p);
                let mut y3 = FiniteField::mul(&s, &x1_minus_x3, &self.p);
                y3 = FiniteField::subs(&y3,y1, &self.p);
                Point::Coor(x3, y3)
            } 
        }

    }

    pub fn scalar_mul(&self, c: &Point, exponent: &BigUint) -> Point{
        let mut t = c.clone(); 
        for i in (0..exponent.bits()-1).rev(){
            t = self.double(&t);
            if exponent.bit(i){
                t = self.add(&t, c);
            } 
        }
        t
    }

    pub fn is_on_curve(&self, c: &Point) -> bool {
        // y^2 = x^3 + ax + b 
        match c {
            Point::Coor(x,y ) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p); 
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mul(x, &self.a, &self.p);
                let mut rhs = FiniteField::add(&x3, &ax, &self.p);
                rhs = FiniteField::add(&rhs, &self.b, &self.p);

                y2 == rhs 
            }
            Point::Identity => true,
        }
    }
}


pub struct FiniteField {

}

impl FiniteField {

    pub fn add(c: &BigUint, d: &BigUint, p:&BigUint) -> BigUint{
        let r = c + d; 
        r.modpow(&BigUint::from(1u32),p)
    }

    pub fn subs(c: &BigUint, d: &BigUint, p:&BigUint) -> BigUint{
        let d_inv = FiniteField::inv_add(d, p);
        FiniteField::add(c, &d_inv, p)
    }

    pub fn mul(c: &BigUint, d: &BigUint, p:&BigUint) -> BigUint{
        let r = c * d;
        r.modpow(&BigUint::from(1u32),p)
    }

    pub fn div(c: &BigUint, d: &BigUint, p:&BigUint) -> BigUint{
        let d_inv = FiniteField::inv_mul(d, p);
        FiniteField::mul(c, &d_inv, p)
    }

    pub fn inv_add(c: &BigUint, p:&BigUint) -> BigUint{
        assert!(c < p);
        p - c 
    }

    pub fn inv_mul(c: &BigUint, p:&BigUint) -> BigUint{
        // it works only p is prime 
        c.modpow(&(p-BigUint::from(2u32)), p)
    }
}


mod test{
    use super::*;

    #[test]
    fn test_add(){
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(11u32);

        let r = FiniteField::add(&c, &d, &p);

        assert_eq!(r, BigUint::from(3u32))
    }

    #[test]
    fn test_mul(){
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(11u32);

        let r = FiniteField::mul(&c, &d, &p);

        assert_eq!(r, BigUint::from(7u32))
    }


    #[test]
    fn test_inv_add(){
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(11u32);

        let r = FiniteField::inv_add(&c, &p);

        assert_eq!(r, BigUint::from(7u32))
    }

    #[test]
    fn test_inv_mul(){
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(11u32);

        let r = FiniteField::inv_mul(&c, &p);

        assert_eq!(r, BigUint::from(3u32))
    }


    #[test]
    fn test_point_add1(){
        //y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve{
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // calculating some points https://www.graui.de/code/elliptic2/
        // put a = 2 and b = 2 then choose a point
        // (6,3) + (3,16) = (6,14)

        let p = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));
        let q = Point::Coor(BigUint::from(3u32), BigUint::from(16u32));

        let r = Point::Coor(BigUint::from(6u32), BigUint::from(14u32));

        let r_prime = ec.add(&p, &q);
        assert_eq!(r,r_prime);
   
    }

    #[test]
    fn test_point_add_2_identitiy(){ //problem 
        //y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve{
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // calculating some points https://www.graui.de/code/elliptic2/
        // put a = 2 and b = 2 then choose a point
        // (3,16) + (3,1) = infinity

        let p = Point::Coor(BigUint::from(3u32), BigUint::from(16u32));
        let q = Point::Coor(BigUint::from(3u32), BigUint::from(1u32));

        let r = Point::Identity;

        let r_prime = ec.add(&p, &q);
        assert_eq!(r,r_prime);
   
    }

    #[test]
    fn test_point_add_3_identitiy(){ 
        //y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve{
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // calculating some points https://www.graui.de/code/elliptic2/
        // put a = 2 and b = 2 then choose a point
        // (3,16) + identity = (1,16)

        let p = Point::Coor(BigUint::from(3u32), BigUint::from(16u32));
        let q = Point::Identity;

        let r = Point::Coor(BigUint::from(3u32), BigUint::from(16u32));

        let r_prime = ec.add(&p, &q);
        assert_eq!(r,r_prime);
   
    }

    #[test]
    fn test_point_double_with_zero_y(){ 
        //y^2 = x^3 + 3x + 2 mod 23
        let ec = EllipticCurve{
            a: BigUint::from(3u32),
            b: BigUint::from(2u32),
            p: BigUint::from(23u32),
        };

        // calculating some points https://www.graui.de/code/elliptic2/
        // put a = 3 and b = 2 in mod 23 because "//y^2 = x^3 + 2x + 2 mod 17" does not have (x,0) point on curve then choose a point
        // choose P (18,0)

        // 2*P where P = (18, 0);
        let p = Point::Coor(BigUint::from(18u32), BigUint::from(0u32));

        let r = Point::Identity;

        let two_p = ec.double(&p);
        assert_eq!(r,two_p);
   
    }

    #[test]
    fn test_point_double(){ 
        //y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve{
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // calculating some points https://www.graui.de/code/elliptic2/
        // put a = 2 and b = 2 then choose a point
        // (6,3) + (6,3) = 2*(6,3) = (3,1)

        let p = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));

        let r = Point::Coor(BigUint::from(3u32), BigUint::from(1u32));

        let two_p = ec.double(&p);
        assert_eq!(r,two_p);
   
    }

    #[test]
    fn test_point_double_identity(){ 
        //y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve{
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // calculating some points https://www.graui.de/code/elliptic2/
        // put a = 2 and b = 2 then choose a point
        // identity + identity = 2*identity = identity 

        let p = Point::Identity;

        let r = Point::Identity;

        let two_p = ec.double(&p);
        assert_eq!(r,two_p);
    }

    #[test]
    fn test_scalar_multiplication(){ 
        //y^2 = x^3 + 2x + 2 mod 17 
        // for any A, 19 * A == Identity because the order of curve is 19 
        let ec = EllipticCurve{
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // 2 * (5,1) = (6,3)
        let p = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));

        let two_p = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));

        let two_p_prime = ec.scalar_mul(&p, &BigUint::from(2u32));
        assert_eq!(two_p,two_p_prime);
    }

    #[test]
    fn test_scalar_multiplication_identitiy(){ 
        //y^2 = x^3 + 2x + 2 mod 17 
        // for any A, 19 * A == Identity because the order of curve is 19 
        let ec = EllipticCurve{
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // 19 * (5,1) = (6,3)
        let p = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));

        let two_p = Point::Identity;

        let two_p_prime = ec.scalar_mul(&p, &BigUint::from(19u32));
        assert_eq!(two_p,two_p_prime);
    }

    #[test]
    fn test_secp256k1(){ 
        //y^2 = x^3 + 7 mod 
        // for any A, 19 * A == Identity because the order of curve is 19 

        //modulo of the field
        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 
            16
        ).unwrap();

        //order of the curve
        let n = BigUint::parse_bytes(
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


        //creating elliptic curve
        let ec = EllipticCurve{
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,
        };

        let g = Point::Coor(generator_x, generator_y);

        let res = ec.scalar_mul(&g, &n);

        assert_eq!(res, Point::Identity);

        let g_double = ec.scalar_mul(&g, &BigUint::from(3u32));

        println!("{:2x?}", &g_double);


    }
}