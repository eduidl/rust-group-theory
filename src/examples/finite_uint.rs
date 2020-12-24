use crate::traits::{AbelianGroup, Group, Monoid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FiniteUint(u8);

pub struct FiniteUintAdd;

impl Monoid for FiniteUintAdd {
    type Element = FiniteUint;

    fn call(a: FiniteUint, b: FiniteUint) -> FiniteUint {
        FiniteUint(a.0.wrapping_add(b.0))
    }

    fn identity() -> FiniteUint {
        FiniteUint(0)
    }
}

impl Group for FiniteUintAdd {
    fn inverse(v: FiniteUint) -> FiniteUint {
        FiniteUint(0u8.wrapping_sub(v.0))
    }
}

impl AbelianGroup for FiniteUintAdd {}

crate::impl_add_group!(FiniteUint, FiniteUintAdd);

#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_finit_uint() {
        assert_eq!(FiniteUintAdd::identity(), FiniteUint(0));

        assert_eq!(-FiniteUint(56), FiniteUint(200));
        assert_eq!(FiniteUint(255) + FiniteUint(3), FiniteUint(2));
        assert_eq!(FiniteUint(3) - FiniteUint(4), FiniteUint(255));

        let mut rng = rand::thread_rng();
        let samples: Vec<_> = (0..1000).map(|_| FiniteUint(rng.gen())).collect();
        <FiniteUintAdd as Group>::test(&samples);
    }
}
