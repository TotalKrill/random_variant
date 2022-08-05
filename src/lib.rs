pub use rand::{self, Rng};
pub use random_variant_macro::*;

mod std_impls;
pub use std_impls::*;

/// Trait that returns a random variant of the given type, if you are creating a
/// new type to limit some values, instead of deriving the RandomVariant,
///
/// implement it manually to ensure only correct values are created
/// Unless you also want to try the Result type
///
/// Useful for hunting down unwraps or other assumptions in your code,
/// The benefit over the crate EveryVariant is that this can be limited to
/// A much smaller set to test, where EveryVariant quickly can grow to unmanageable
/// testing sets
pub trait RandomVariant {
    fn random_variant<R: Rng>(rng: &mut R) -> Self;
}

/// An example of a runtime failure that is encountered
#[allow(dead_code)]
#[cfg(test)]
fn example() {
    use serde::Serialize;
    use std::net::IpAddr;

    #[derive(RandomVariant, Serialize)]
    enum Message {
        Log(String),
        ErrorCode(u32),
        /// This tuple will fail to serialize if flattened in serde
        Rebooting,
    }

    #[derive(RandomVariant, Serialize)]
    struct LoggedMessage {
        /// Fail to serialize, since flatteing enums is not supported
        #[serde(flatten)]
        t: Message,
        add: IpAddr,
    }
    let mut rng = crate::rand::thread_rng();

    for _i in 0..100 {
        let v = LoggedMessage::random_variant(&mut rng);
        serde_json::to_string(&v).unwrap();
    }
}

#[cfg(test)]
mod test;
