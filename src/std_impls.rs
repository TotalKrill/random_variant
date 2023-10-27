use std::{
    collections::{BTreeMap, BTreeSet},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};

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

impl RandomVariant for SocketAddr {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        Self::new(
            RandomVariant::random_variant(rng),
            RandomVariant::random_variant(rng),
        )
    }
}

impl RandomVariant for IpAddr {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..2);
        match u {
            0 => IpAddr::V4(RandomVariant::random_variant(rng)),
            1 => IpAddr::V6(RandomVariant::random_variant(rng)),
            _ => panic!(),
        }
    }
}
impl RandomVariant for Ipv6Addr {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: u128 = rng.gen();
        u.into()
    }
}

impl RandomVariant for Ipv4Addr {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: u32 = rng.gen();
        u.into()
    }
}

impl<T: RandomVariant + Default + Copy, const N: usize> RandomVariant for [T; N] {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let mut arry: [T; N] = [Default::default(); N];
        for i in 0..N {
            arry[i] = T::random_variant(rng);
        }
        arry
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

impl_random!(
    bool,
    (),
    i8,
    i16,
    i32,
    i64,
    i128,
    u8,
    u16,
    u32,
    u64,
    u128,
    f32,
    f64,
    usize,
    char
);

macro_rules! impl_nonzeroes{
    ($($t:ty),* ) => {
        $(
            impl RandomVariant for $t {
                fn random_variant<R: Rng>(rng: &mut R) -> Self {
                    let mut i = rng.gen();
                    loop {
                        if let Some(val) = <$t>::new(i) {
                            return val;
                        } else {
                            i = rng.gen();
                        }
                    }
                }
            }
            )*
    };
}

impl_nonzeroes!(
    std::num::NonZeroI128,
    std::num::NonZeroI64,
    std::num::NonZeroI32,
    std::num::NonZeroI16,
    std::num::NonZeroI8,
    std::num::NonZeroU128,
    std::num::NonZeroU64,
    std::num::NonZeroU32,
    std::num::NonZeroU16,
    std::num::NonZeroU8
);

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

use std::collections::{HashMap, HashSet};

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
impl<K, V> RandomVariant for BTreeMap<K, V>
where
    K: RandomVariant + core::hash::Hash + Eq + Ord,
    V: RandomVariant + core::hash::Hash + Eq + Ord,
{
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..100);
        let mut hashset = BTreeMap::default();
        for _i in 0..u {
            let _e = hashset.insert(K::random_variant(rng), V::random_variant(rng));
        }
        hashset
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
impl<T: RandomVariant> RandomVariant for Vec<T> {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let mut s = Vec::new();
        let val = rng.gen_range(0..100);
        for _ in 0..val {
            s.push(T::random_variant(rng))
        }
        s
    }
}
impl<V> RandomVariant for HashSet<V>
where
    V: RandomVariant + core::hash::Hash + Eq,
{
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..100);
        let mut hashset = HashSet::default();
        for _i in 0..u {
            let _e = hashset.insert(V::random_variant(rng));
        }
        hashset
    }
}

impl<V> RandomVariant for BTreeSet<V>
where
    V: RandomVariant + core::hash::Hash + Eq + Ord,
{
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..100);
        let mut hashset = BTreeSet::default();
        for _i in 0..u {
            let _e = hashset.insert(V::random_variant(rng));
        }
        hashset
    }
}
