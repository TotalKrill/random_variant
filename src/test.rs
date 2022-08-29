#[cfg(test)]
mod tests {
    use crate::RandomVariant;
    use rand::thread_rng;

    /// Type of the message
    #[derive(RandomVariant, Debug, Clone)]
    enum MessageType {
        Codified,
        Markdown,
        Html,
    }

    /// This type can come in  4 different variants due the option
    #[derive(RandomVariant, Debug, Clone)]
    struct FormattedMessage {
        /// Enum dictating how to render the string, None means its hidden
        #[allow(dead_code)]
        rendermethod: Option<MessageType>,
        /// The optional content of the message
        #[allow(dead_code)]
        text: String,
    }

    #[test]
    fn small_example() {
        let mut rng = thread_rng();
        for _i in 0..100 {
            let u: i32 = RandomVariant::random_variant(&mut rng);
            println!("{:?}", u);
            let u: (i32, u32) = RandomVariant::random_variant(&mut rng);
            println!("{:?}", u);
        }
        let all_diferent_messages = FormattedMessage::random_variant(&mut rng);
        println!("{:#?}", all_diferent_messages);

        let _opt_msg_len = Option::<MessageType>::random_variant(&mut rng);
        let _text_len = String::random_variant(&mut rng).len();
    }

    #[derive(RandomVariant, Debug, Clone)]
    pub struct Message {
        pub message: String,
        pub number: u32,
        pub opt: Option<u64>,
        pub nest: Top,
        pub second: SecondTop,
    }

    #[derive(RandomVariant, Debug, Clone)]
    pub enum SecondTop {
        One,
        Two(Nested),
        Three,
    }

    #[derive(RandomVariant, Debug, Clone)]
    pub enum Top {
        One,
        Nested(Nested),
    }

    #[derive(RandomVariant, Debug, Clone)]
    pub enum Nested {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
        Sixt,
        Seventh,
        Eight,
    }

    #[derive(RandomVariant, Debug, Clone)]
    pub struct TestUnnamed3(pub u16);

    // #[derive(RandomVariant, Debug, Clone)]
    // pub struct TestUnnamedBorrow<'a>(pub &'a str);

    #[derive(RandomVariant, Debug, Clone)]
    pub enum TestUnnamed1 {
        UnnamedSingle(u16),
        UnnamedMultiple1(u16, u32),
        UnnamedMultiple2(u16, u32, u64, i32),
    }

    #[derive(RandomVariant, Debug, Clone)]
    pub struct TestUnnamed2(u16, u32, u64);

    #[derive(RandomVariant, Debug, Clone)]
    pub struct Gen1<A: RandomVariant + Clone>(A);

    #[derive(RandomVariant, Debug, Clone)]
    pub struct Gen2<A: RandomVariant + Clone, B: RandomVariant + Clone>(A, B);

    #[derive(RandomVariant, Debug, Clone)]
    pub enum Generic2 {
        G1(Gen1<i8>),
        G2(Gen2<i16, i32>),
    }

    #[derive(RandomVariant, Debug, Clone)]
    pub enum TestNamed1 {
        NamedSingle { first: u16 },
        NamedMultiple1 { first: u16, second: u32 },
        NamedMultiple2 { first: u16, second: u32, third: u64 },
    }

    #[allow(unused)]
    #[derive(RandomVariant)]
    pub struct GenericDerive<T> {
        value: T,
    }

    #[allow(unused)]
    #[derive(RandomVariant)]
    pub enum GenericEnum<T> {
        One(T),
        Two(u32),
    }

    #[allow(unused)]
    #[derive(RandomVariant)]
    pub struct MultiGeneric<A, B>(A, B);
}
