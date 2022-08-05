#[cfg(test)]
mod tests {
    use crate::RandomVariant;
    use crate::Test;
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
        rendermethod: Option<MessageType>,
        /// The optional content of the message
        text: String,
    }

    #[test]
    fn small_example() {
        let mut rng = thread_rng();
        for _i in 0..100 {
            let u: Test = RandomVariant::random_variant(&mut rng);
            println!("{:?}", u);
            let u: i32 = RandomVariant::random_variant(&mut rng);
            println!("{:?}", u);
            let u: (i32, u32) = RandomVariant::random_variant(&mut rng);
            println!("{:?}", u);
            let u: Result<Test, ()> = RandomVariant::random_variant(&mut rng);
            println!("{:?}", u);
        }
        let all_diferent_messages = FormattedMessage::random_variant(&mut rng);
        println!("{:#?}", all_diferent_messages);

        let opt_msg_len = Option::<MessageType>::random_variant(&mut rng);
        let text_len = String::random_variant(&mut rng).len();
    }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub struct Message {
    //         pub message: String,
    //         pub number: u32,
    //         pub opt: Option<u64>,
    //         pub nest: Top,
    //         pub second: SecondTop,
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub enum SecondTop {
    //         One,
    //         Two(Nested),
    //         Three,
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub enum Top {
    //         One,
    //         Nested(Nested),
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub enum Nested {
    //         First,
    //         Second,
    //         Third,
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub struct TestUnnamed3(pub u16);

    //     #[test]
    //     fn messages_number() {
    //         let msgs = Message::every_variant().len();
    //         let messages_len = String::every_variant().len();
    //         let number_len = u32::every_variant().len();
    //         let opt_len = Option::<u64>::every_variant().len();
    //         let nest_len = Top::every_variant().len();
    //         let second_len = SecondTop::every_variant().len();

    //         assert_eq!(
    //             (messages_len * number_len * opt_len * nest_len * second_len),
    //             msgs
    //         );
    //     }

    //     #[test]
    //     fn opts_number() {
    //         let msgs = Option::<u64>::every_variant().len();
    //         assert_eq!(u64::every_variant().len() + 1, msgs);
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub enum TestUnnamed1 {
    //         UnnamedSingle(u16),
    //         UnnamedMultiple1(u16, u32),
    //         UnnamedMultiple2(u16, u32, u64, i32),
    //     }

    //     #[test]
    //     fn unnamed1() {
    //         let msgs = TestUnnamed1::every_variant().len();
    //         let u16_len = u16::every_variant().len();
    //         let u32_len = u32::every_variant().len();
    //         let u64_len = u64::every_variant().len();
    //         let i32_len = i32::every_variant().len();
    //         assert_eq!(
    //             u16_len + u16_len * u32_len + u16_len * u32_len * u64_len * i32_len,
    //             msgs
    //         );
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub struct TestUnnamed2(u16, u32, u64);

    //     #[test]
    //     fn unnamed2() {
    //         let msgs = TestUnnamed2::every_variant().len();
    //         let u16_len = u16::every_variant().len();
    //         let u32_len = u32::every_variant().len();
    //         let u64_len = u64::every_variant().len();
    //         assert_eq!(u16_len * u32_len * u64_len, msgs);
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub struct Gen1<A: EveryVariant + Clone>(A);

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub struct Gen2<A: EveryVariant + Clone, B: EveryVariant + Clone>(A, B);

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub struct Generic1(Gen1<u8>, Gen2<u16, u32>);
    //     #[test]
    //     fn generic1() {
    //         let msgs = Generic1::every_variant().len();

    //         let gen1_len = Gen1::<u8>::every_variant().len();
    //         let gen2_len = Gen2::<u16, u32>::every_variant().len();

    //         assert_eq!(gen1_len * gen2_len, msgs);
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub enum Generic2 {
    //         G1(Gen1<i8>),
    //         G2(Gen2<i16, i32>),
    //     }

    //     #[test]
    //     fn generic2() {
    //         let msgs = Generic2::every_variant().len();
    //         let gen1_len = Gen1::<i8>::every_variant().len();
    //         let gen2_len = Gen2::<i16, i32>::every_variant().len();
    //         assert_eq!(gen1_len + gen2_len, msgs);
    //     }

    //     #[derive(EveryVariant, Debug, Clone)]
    //     pub enum TestNamed1 {
    //         NamedSingle { first: u16 },
    //         NamedMultiple1 { first: u16, second: u32 },
    //         NamedMultiple2 { first: u16, second: u32, third: u64 },
    //     }

    //     #[test]
    //     fn named_enum() {
    //         let msgs = TestNamed1::every_variant().len();
    //         let u16_len = u16::every_variant().len();
    //         let u32_len = u32::every_variant().len();
    //         let u64_len = u64::every_variant().len();
    //         assert_eq!(
    //             u16_len + u16_len * u32_len + u16_len * u32_len * u64_len,
    //             msgs
    //         );
    //     }

    //     #[allow(unused)]
    //     #[derive(EveryVariant)]
    //     pub struct GenericDerive<T> {
    //         value: T,
    //     }

    //     #[allow(unused)]
    //     #[derive(EveryVariant)]
    //     pub enum GenericEnum<T> {
    //         One(T),
    //         Two(u32),
    //     }

    //     #[allow(unused)]
    //     #[derive(EveryVariant)]
    //     pub struct MultiGeneric<A, B>(A, B);

    //     #[test]
    //     fn generic_everyvariant() {
    //         let msgs = GenericDerive::<u32>::every_variant().len();
    //         assert_eq!(u32::every_variant().len(), msgs);

    //         let msgs = GenericEnum::<u32>::every_variant().len();
    //         assert_eq!(2 * u32::every_variant().len(), msgs);

    //         let msgs = MultiGeneric::<u32, u32>::every_variant().len();
    //         assert_eq!(
    //             u32::every_variant().len() * u32::every_variant().len(),
    //             msgs
    //         );
    //     }
}
