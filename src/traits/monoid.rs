use std::fmt::Debug;

use itertools::Itertools;

pub trait Monoid {
    type Element: Clone + Debug + Eq;

    fn call(a: Self::Element, b: Self::Element) -> Self::Element;

    fn identity() -> Self::Element;

    fn test_associative_property(a: &Self::Element, b: &Self::Element, c: &Self::Element) {
        assert_eq!(
            Self::call(Self::call(a.clone(), b.clone()), c.clone()),
            Self::call(a.clone(), Self::call(b.clone(), c.clone()))
        );
    }

    fn test_identity_element(v: &Self::Element) {
        assert_eq!(Self::call(v.clone(), Self::identity()), v.clone());
        assert_eq!(Self::call(Self::identity(), v.clone()), v.clone());
    }

    fn test(samples: &[Self::Element]) {
        for (a, b, c) in samples.iter().tuple_windows() {
            Self::test_associative_property(a, b, c);
        }

        for v in samples {
            Self::test_identity_element(v);
        }
    }
}
