use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::traits::Monoid;

pub trait Group: Monoid {
    fn inverse(v: Self::Element) -> Self::Element;

    fn test_inverse_element(v: &Self::Element) {
        let inverse = Self::inverse(v.clone());
        assert_eq!(Self::call(v.clone(), inverse.clone()), Self::identity());
        assert_eq!(Self::call(inverse, v.clone()), Self::identity());
    }

    fn test(samples: &[Self::Element]) {
        <Self as Monoid>::test(samples);
        samples.iter().for_each(Self::test_inverse_element);
    }
}

pub trait GroupExcludeZero: Monoid {
    type AddOp: Monoid<Element = Self::Element>;

    fn inverse_impl(v: Self::Element) -> Self::Element;

    fn inverse(v: Self::Element) -> Result<Self::Element> {
        if v == Self::AddOp::identity() {
            Err(anyhow!(
                "Exceptionally `0` has no inverse element for `multiplication`"
            ))
        } else {
            Ok(Self::inverse_impl(v))
        }
    }

    fn test_inverse_element(v: &Self::Element) {
        match Self::inverse(v.clone()) {
            Ok(inverse) => {
                assert_eq!(Self::call(v.clone(), inverse.clone()), Self::identity());
                assert_eq!(Self::call(inverse, v.clone()), Self::identity());
            }
            Err(_) => {
                assert_eq!(*v, Self::AddOp::identity());
            }
        }
    }

    fn test(samples: &[Self::Element]) {
        <Self as Monoid>::test(samples);
        samples.iter().for_each(Self::test_inverse_element);
    }
}

pub trait AbelianGroup: Group {
    fn test_commutative_law(a: &Self::Element, b: &Self::Element) {
        assert_eq!(
            Self::call(a.clone(), b.clone()),
            Self::call(b.clone(), a.clone())
        );
    }

    fn test(samples: &[Self::Element]) {
        <Self as Group>::test(samples);
        samples
            .iter()
            .tuple_windows()
            .for_each(|(a, b)| Self::test_commutative_law(a, b))
    }
}
