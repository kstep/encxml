
extern crate "rustc-serialize" as rustc_serialize;
extern crate xml;

use std::collections::BTreeMap;
use std::io::{Buffer, Reader};
use xml::reader::EventReader;
use xml::common::Error as XmlError;
use rustc_serialize::Decoder;

struct XmlDecoder<B: Buffer> {
    reader: EventReader<B>
}

impl<B: Buffer> XmlDecoder<B> {
    fn new(buffer: B) -> XmlDecoder<B> {
        XmlDecoder {
            reader: EventReader::new(buffer)
        }
    }
}

impl<B: Buffer> Decoder for XmlDecoder<B> {
    type Error = XmlError;
    fn read_nil(&mut self) -> Result<(), XmlError> {
    }

    fn read_usize(&mut self) -> Result<usize, XmlError> {
    }

    fn read_u64(&mut self) -> Result<u64, XmlError> {
    }

    fn read_u32(&mut self) -> Result<u32, XmlError> {
    }

    fn read_u16(&mut self) -> Result<u16, XmlError> {
    }

    fn read_u8(&mut self) -> Result<u8, XmlError> {
    }

    fn read_isize(&mut self) -> Result<isize, XmlError> {
    }

    fn read_i64(&mut self) -> Result<i64, XmlError> {
    }

    fn read_i32(&mut self) -> Result<i32, XmlError> {
    }

    fn read_i16(&mut self) -> Result<i16, XmlError> {
    }

    fn read_i8(&mut self) -> Result<i8, XmlError> {
    }

    fn read_bool(&mut self) -> Result<bool, XmlError> {
    }

    fn read_f64(&mut self) -> Result<f64, XmlError> {
    }

    fn read_f32(&mut self) -> Result<f32, XmlError> {
    }

    fn read_char(&mut self) -> Result<char, XmlError> {
    }

    fn read_str(&mut self) -> Result<String, XmlError> {
    }

    fn read_enum<T, F>(&mut self, name: &str, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, XmlError> where F: FnMut(&mut Self, usize) -> Result<T, XmlError> {
    }

    fn read_enum_variant_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, XmlError> where F: FnMut(&mut Self, usize) -> Result<T, XmlError> {
    }

    fn read_enum_struct_variant_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_struct_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_tuple<T, F>(&mut self, len: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_tuple_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_tuple_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_option<T, F>(&mut self, f: F) -> Result<T, XmlError> where F: FnMut(&mut Self, bool) -> Result<T, XmlError> {
    }

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self, usize) -> Result<T, XmlError> {
    }

    fn read_seq_elt<T, F>(&mut self, idx: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_map<T, F>(&mut self, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self, usize) -> Result<T, XmlError> {
    }

    fn read_map_elt_key<T, F>(&mut self, idx: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn read_map_elt_val<T, F>(&mut self, idx: usize, f: F) -> Result<T, XmlError> where F: FnOnce(&mut Self) -> Result<T, XmlError> {
    }

    fn error(&mut self, err: &str) -> XmlError {
        XmlError::new_full(0, 0, err.to_string())
    }

}


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
        let mut decoder = XmlDecoder::new(reader);
        assert_eq!(SomeStruct {
            someint: 123,
            somestr: "a string".to_string(),
            somevec: vec![1, 2, 3],
            someopt: Some(100)
        }, Decodable::decode(&mut decoder).unwrap());
    }
}
