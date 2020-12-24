use anyhow::Result;
use itertools::Itertools;

use crate::traits::{AbelianGroup, GroupExcludeZero, Ring};

pub trait Field: Ring
where
    Self::MulOp: GroupExcludeZero<Element = Self::Element>,
{
    fn reciprocal(v: Self::Element) -> Result<Self::Element> {
        Self::MulOp::inverse(v)
    }

    fn div(a: Self::Element, b: Self::Element) -> Result<Self::Element> {
        Ok(Self::mul(a, Self::reciprocal(b)?))
    }

    fn test(samples: &[Self::Element]) {
        <Self::AddOp as AbelianGroup>::test(samples);
        Self::MulOp::test(samples);
        samples
            .iter()
            .tuple_windows()
            .for_each(|(a, b, c)| Self::test_distributive_property(a, b, c))
    }
}
