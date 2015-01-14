
extern crate "rustc-serialize" as rustc_serialize;
extern crate xml;

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use rustc_serialize::{Encoder, Decoder, Encodable, Decodable};

    use super::XmlDecoder;

    #[derive(PartialEq, RustcDecodable, RustcEncodable)]
    struct SomeStruct {
        someint: u32,
        somestr: String,
        somevec: Vec<u32>,
        someopt: Option<u32>
    }

    const XML_STRUCT: &'static str = r#"
    <?xml version="1.0"?>
    <SomeStruct>
        <someint>123</someint>
        <somestr>a string</somestr>
        <somevec>1</somevec>
        <somevec>2</somevec>
        <somevec>3</somevec>
        <someopt>100</someopt>
    </SomeStruct>
    "#;

    #[test]
    fn test_decode() {
        let mut reader = BufReader::new(XML_STRUCT.as_bytes());
        let mut decoder = XmlDecoder::new(&mut reader);
        assert_eq!(SomeStruct {
            someint: 123,
            somestr: "a string".to_string(),
            somevec: vec![1, 2, 3],
            someopt: Some(100)
        }, Decodable::decode(&mut decoder).unwrap());
    }
}
