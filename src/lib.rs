pub use rand::{self, Rng};
pub use random_variant_macro::*;

mod std_impls;
pub use std_impls::*;

pub trait RandomVariant {
    fn random_variant<R: Rng>(rng: &mut R) -> Self; // {rng.gen()};
}

mod test;
