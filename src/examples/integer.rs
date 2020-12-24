use num_bigint::BigInt;

use crate::traits::{AbelianGroup, Group, Monoid, Ring};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Z(BigInt);

impl Z {
    pub fn new(v: impl Into<BigInt>) -> Self {
        Self(v.into())
    }
}

pub struct ZAdd;

impl Monoid for ZAdd {
    type Element = Z;

    fn call(a: Z, b: Z) -> Z {
        Z(a.0 + b.0)
    }

    fn identity() -> Z {
        Z::new(0)
    }
}

impl Group for ZAdd {
    fn inverse(v: Z) -> Z {
        Z(-v.0)
    }
}

impl AbelianGroup for ZAdd {}

struct ZMul;

impl Monoid for ZMul {
    type Element = Z;

    fn call(a: Z, b: Z) -> Z {
        Z(a.0 * b.0)
    }

    fn identity() -> Z {
        Z(1.into())
    }
}

struct ZRing;

impl Ring for ZRing {
    type Element = Z;
    type AddOp = ZAdd;
    type MulOp = ZMul;
}

crate::impl_ring!(Z, ZRing);

#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_integer() {
        assert_eq!(ZRing::zero(), Z(0.into()));
        assert_eq!(ZRing::one(), Z(1.into()));

        assert_eq!(-Z::new(2), Z::new(-2));
        assert_eq!(Z::new(3) + Z::new(5), Z::new(8));
        assert_eq!(Z::new(3) - Z::new(5), Z::new(-2));
        assert_eq!(Z::new(3) * Z::new(5), Z::new(15));

        let mut rng = rand::thread_rng();
        let mut samples = vec![Z::new(1), Z::new(0)];
        for _ in 0..1000 {
            samples.push(Z::new(rng.gen::<i64>()));
        }
        ZRing::test_ring(&samples);
    }
}
