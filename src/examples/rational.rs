use std::fmt::Debug;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::traits::{AbelianGroup, Field, Group, GroupExcludeZero, Monoid, Ring};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Q(BigRational);

impl Q {
    pub fn new(numer: impl Into<BigInt>, denom: impl Into<BigInt>) -> Self {
        Self(BigRational::new(numer.into(), denom.into()))
    }

    pub fn integer(v: impl Into<BigInt>) -> Self {
        Self(BigRational::from_integer(v.into()))
    }
}

pub struct QAdd;

impl Monoid for QAdd {
    type Element = Q;

    fn call(a: Q, b: Q) -> Q {
        Q(a.0 + b.0)
    }

    fn identity() -> Q {
        Q::integer(0)
    }
}

impl Group for QAdd {
    fn inverse(v: Q) -> Q {
        Q(-v.0)
    }
}

impl AbelianGroup for QAdd {}

struct QMul;

impl Monoid for QMul {
    type Element = Q;

    fn call(a: Q, b: Q) -> Q {
        Q(a.0 * b.0)
    }

    fn identity() -> Q {
        Q::integer(1)
    }
}

impl GroupExcludeZero for QMul {
    type AddOp = QAdd;

    fn inverse_impl(v: Q) -> Q {
        Q(v.0.recip())
    }
}

struct QField;

impl Ring for QField {
    type Element = Q;
    type AddOp = QAdd;
    type MulOp = QMul;
}

impl Field for QField {}

crate::impl_field!(Q, QField);

#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_rational() {
        assert_eq!(QField::zero(), Q::integer(0));
        assert_eq!(QField::one(), Q::integer(1));

        assert_eq!(-Q::new(1, 3), Q::new(-1, 3));
        assert_eq!(QField::reciprocal(Q::new(1, 3)).unwrap(), Q::integer(3));
        assert_eq!(Q::new(1, 3) + Q::new(1, 5), Q::new(8, 15));
        assert_eq!(Q::new(1, 3) - Q::new(1, 5), Q::new(2, 15));
        assert_eq!(Q::new(1, 3) * Q::new(1, 5), Q::new(1, 15));
        assert_eq!(Q::new(1, 3) / Q::new(1, 5), Q::new(5, 3));

        let mut rng = rand::thread_rng();
        let mut samples = vec![Q::integer(1), Q::integer(0)];
        for _ in 0..1000 {
            let numer = rng.gen::<i64>();
            let denom = rng.gen_range(1..=std::i64::MAX);
            samples.push(Q::new(numer, denom));
        }
        QField::test(&samples);
    }
}
