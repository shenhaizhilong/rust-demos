#[cfg(test)]
pub mod tests {
    #[test]
    fn test1() {
        let a = hex::encode("Hello world!");
        println!("hex(a)={a}");
        // assert_eq!(hex::encode("Hello world!"), "48656c6c6f20776f726c6421");
        assert_eq!(hex::encode_upper("Hello world!"), "48656C6C6F20776F726C6421");
        let b = hex::decode("48656c6c6f20776f726c6421").unwrap();
        let c = String::from_utf8(b).unwrap();
        println!("decode(48656c6c6f20776f726c6421)={c}");
    }

    #[test]
    fn test2() {
        let mut a = [0u8; 4];
        hex::decode_to_slice("6b697769", &mut a as &mut [u8]).expect("TODO: panic message");
        println!("{}", String::from_utf8(Vec::from(a)).unwrap());
    }
}