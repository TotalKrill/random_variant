use crate::{RandomVariant, Rng};
use uuid::Uuid;

impl RandomVariant for Uuid {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        Self::from_u128(rng.gen())
    }
}
