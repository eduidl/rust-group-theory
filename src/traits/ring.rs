use std::fmt::Debug;

use itertools::Itertools;

use crate::traits::{AbelianGroup, Group, Monoid};

pub trait Ring {
    type Element: Clone + Debug + Eq;
    type AddOp: AbelianGroup<Element = Self::Element>;
    type MulOp: Monoid<Element = Self::Element>;

    fn zero() -> Self::Element {
        Self::AddOp::identity()
    }

    fn one() -> Self::Element {
        Self::MulOp::identity()
    }

    fn minus(v: Self::Element) -> Self::Element {
        Self::AddOp::inverse(v)
    }

    fn add(a: Self::Element, b: Self::Element) -> Self::Element {
        Self::AddOp::call(a, b)
    }

    fn sub(a: Self::Element, b: Self::Element) -> Self::Element {
        Self::AddOp::call(a, Self::minus(b))
    }

    fn mul(a: Self::Element, b: Self::Element) -> Self::Element {
        Self::MulOp::call(a, b)
    }

    fn test_distributive_property(a: &Self::Element, b: &Self::Element, c: &Self::Element) {
        assert_eq!(
            Self::mul(Self::add(a.clone(), b.clone()), c.clone()),
            Self::add(
                Self::mul(a.clone(), c.clone()),
                Self::mul(b.clone(), c.clone())
            )
        );
        assert_eq!(
            Self::mul(a.clone(), Self::add(b.clone(), c.clone())),
            Self::add(
                Self::mul(a.clone(), b.clone()),
                Self::mul(a.clone(), c.clone())
            )
        );
    }

    fn test_ring(samples: &[Self::Element]) {
        <Self::AddOp as AbelianGroup>::test(samples);
        Self::MulOp::test(samples);
        samples
            .iter()
            .tuple_windows()
            .for_each(|(a, b, c)| Self::test_distributive_property(a, b, c))
    }
}
