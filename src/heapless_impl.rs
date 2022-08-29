use crate::*;
use heapless::{String as HString, Vec as HVec};

impl<T, const N: usize> RandomVariant for HVec<T, N>
where
    T: RandomVariant + Clone + Sized,
{
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..=N);
        let mut vec = HVec::new();
        for _i in 0..u {
            let _ = vec.push(T::random_variant(rng));
        }
        vec
    }
}

impl<const N: usize> RandomVariant for HString<N> {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..=N);
        let mut vec = HString::new();
        for _i in 0..u {
            let _ = vec.push(rng.gen());
        }
        vec
    }
}
