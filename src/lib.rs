use rand::Rng;
pub use random_variant_macro::*;

pub trait RandomVariant {
    fn random_variant<R: Rng>(rng: &mut R) -> Self; // {rng.gen()};
}

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

impl RandomVariant for String {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let mut s = String::new();
        let val = rng.gen_range(0..400);
        for _v in 0..val {
            s.push(rng.gen())
        }
        s
        // let u: usize = rng.gen_range(0..=1);
        // match u {
        //     0 => Test::A,
        //     1 => Test::B,
        //     _ => panic!(""),
        // }
    }
}

#[derive(Debug)]
enum Test {
    A,
    B,
}
impl RandomVariant for Test {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        let u: usize = rng.gen_range(0..=1);
        match u {
            0 => Test::A,
            1 => Test::B,
            _ => panic!(""),
        }
    }
}

impl<A: RandomVariant, B: RandomVariant> RandomVariant for (A, B) {
    fn random_variant<R: Rng>(rng: &mut R) -> Self {
        (A::random_variant(rng), B::random_variant(rng))
    }
}

mod test;
