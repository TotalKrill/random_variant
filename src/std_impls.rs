use std::collections::HashMap;

use rand::Rng;

use crate::RandomVariant;

impl<T: RandomVariant> RandomVariant for Option<T> {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..=1);
        match u {
            0 => Some(T::random_variant(rng)),
            1 => None,
            _ => panic!(""),
        }
    }
}

impl<T: RandomVariant, E: RandomVariant> RandomVariant for Result<T, E> {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..=1);
        match u {
            0 => Ok(T::random_variant(rng)),
            1 => Err(E::random_variant(rng)),
            _ => panic!(""),
        }
    }
}

impl<K, V> RandomVariant for HashMap<K, V>
where
    K: RandomVariant + core::hash::Hash + Eq,
    V: RandomVariant,
{
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..100);
        let mut hashmap = HashMap::new();
        for _i in 0..u {
            let _e = hashmap.insert(K::random_variant(rng), V::random_variant(rng));
        }
        hashmap
    }
}
macro_rules! impl_random {
    ($($t:ty),* ) => {
        $(
        impl RandomVariant for $t {
            fn random_variant<R: Rng>(rng: &mut R) -> Self {
                rng.gen()
            }
        }
        )*
    };
}

impl_random!((), i32, u32, i64, u64, f32, f64, i8, u8, u16, i16, char);

macro_rules! impl_random_tuple {
    ($($gen:ident),* ) => {
        impl < $( $gen,  )* > RandomVariant for  ( $($gen, )* )
        where
            $(
                $gen: RandomVariant,
            )*
            {
            fn random_variant<R: Rng>(rng: &mut R) -> Self {
                (
                    $( $gen::random_variant(rng), )*
                )
            }
        }
    };
}

impl_random_tuple!(A, B);
impl_random_tuple!(A, B, C);
impl_random_tuple!(A, B, C, D);
impl_random_tuple!(A, B, C, D, E);
impl_random_tuple!(A, B, C, D, E, G);
impl_random_tuple!(A, B, C, D, E, G, F);
impl_random_tuple!(A, B, C, D, E, G, F, H);
impl_random_tuple!(A, B, C, D, E, G, F, H, I);
impl_random_tuple!(A, B, C, D, E, G, F, H, I, J);
impl_random_tuple!(A, B, C, D, E, G, F, H, I, J, K);
impl_random_tuple!(A, B, C, D, E, G, F, H, I, J, K, L);
impl_random_tuple!(A, B, C, D, E, G, F, H, I, J, K, L, M);

impl RandomVariant for String {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let mut s = String::new();
        let val = rng.gen_range(0..400);
        for _v in 0..val {
            s.push(rng.gen())
        }
        s
    }
}
