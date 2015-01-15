
extern crate "rustc-serialize" as rustc_serialize;
extern crate xml;

use std::collections::BTreeMap;
use std::io::{Buffer, Reader};
use xml::reader::{EventReader, Events};
use xml::reader::events::XmlEvent;
use xml::common::Error as XmlError;
use rustc_serialize::Decoder;

struct XmlDecoder<'a, B: 'a + Buffer> {
    reader: EventReader<B>,
    events: Events<'a, B>
}

impl<'a, B: 'a + Buffer> XmlDecoder<'a, B> {
    fn new(buffer: B) -> XmlDecoder<'a, B> {
        let reader = EventReader::new(buffer);
        let events = reader.events();
        XmlDecoder {
            reader: reader,
            events: events
        }
    }
}

enum XmlDecodeError {
    XmlError(XmlError),
    InvalidFormat,
    UnexpectedTag,
}

impl std::error::FromError<XmlError> for XmlDecodeError {
    fn from_error(err: XmlError) -> XmlDecodeError {
        XmlDecodeError::XmlError(err)
    }
}

impl<'a, B: Buffer> Decoder for XmlDecoder<'a, B> {
    type Error = XmlDecodeError;
    fn read_nil(&mut self) -> Result<(), XmlDecodeError> {
        Ok(())
    }

    fn read_usize(&mut self) -> Result<usize, XmlDecodeError> {
        self.read_u64().map(|v| v as usize)
    }

    fn read_u64(&mut self) -> Result<u64, XmlDecodeError> {
        match self.events.next() {
            XmlEvent::Characters(c) | XmlEvent::CData(c) => match c.parse() {
                None => Err(XmlDecodeError::InvalidFormat),
                Some(v) => Ok(v)
            },
            XmlEvent::Error(e) => Err(XmlDecodeError::XmlError(e)),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn read_u32(&mut self) -> Result<u32, XmlDecodeError> {
        self.read_u64().map(|v| v as u32)
    }

    fn read_u16(&mut self) -> Result<u16, XmlDecodeError> {
        self.read_u64().map(|v| v as u16)
    }

    fn read_u8(&mut self) -> Result<u8, XmlDecodeError> {
        self.read_u64().map(|v| v as u8)
    }

    fn read_isize(&mut self) -> Result<isize, XmlDecodeError> {
        self.read_i64().map(|v| v as isize)
    }

    fn read_i64(&mut self) -> Result<i64, XmlDecodeError> {
        match self.events.next() {
            XmlEvent::Characters(c) | XmlEvent::CData(c) => match c.parse() {
                None => Err(XmlDecodeError::InvalidFormat),
                Some(v) => Ok(v)
            },
            XmlEvent::Error(e) => Err(XmlDecodeError::XmlError(e)),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn read_i32(&mut self) -> Result<i32, XmlDecodeError> {
        self.read_i64().map(|v| v as i32)
    }

    fn read_i16(&mut self) -> Result<i16, XmlDecodeError> {
        self.read_i64().map(|v| v as i16)
    }

    fn read_i8(&mut self) -> Result<i8, XmlDecodeError> {
        self.read_i64().map(|v| v as i8)
    }

    fn read_bool(&mut self) -> Result<bool, XmlDecodeError> {
        self.read_u8().map(|v| v != 0u8)
    }

    fn read_f64(&mut self) -> Result<f64, XmlDecodeError> {
        match self.events.next() {
            XmlEvent::Characters(c) | XmlEvent::CData(c) => match c.parse() {
                None => Err(XmlDecodeError::InvalidFormat),
                Some(v) => Ok(v)
            },
            XmlEvent::Error(e) => Err(XmlDecodeError::XmlError(e)),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn read_f32(&mut self) -> Result<f32, XmlDecodeError> {
        self.read_f64().map(|v| v as f32)
    }

    fn read_char(&mut self) -> Result<char, XmlDecodeError> {
        match self.events.next() {
            XmlEvent::Characters(c) | XmlEvent::CData(c) => match c.parse() {
                None => Err(XmlDecodeError::InvalidFormat),
                Some(v) => Ok(v)
            },
            XmlEvent::Error(e) => Err(XmlDecodeError::XmlError(e)),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn read_str(&mut self) -> Result<String, XmlDecodeError> {
        match self.events.next() {
            XmlEvent::Characters(c) | XmlEvent::CData(c) => c,
            XmlEvent::Error(e) => Err(XmlDecodeError::XmlError(e)),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn read_enum<T, F>(&mut self, name: &str, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        match self.events.next() {
            XmlEvent::StartElement { name, .. } if &*name.local_name = name => {
                let result = f(self);
                match self.events.next() {
                    XmlEvent::EndElement { name } if &*name == name => result,
                    XmlEvent::Error(e) => Err(XmlDecodeError::XmlError(e)),
                    _ => Err(XmlDecodeError::UnexpectedTag)
                }
            },
            XmlEvent::Error(e) => Err(XmlDecodeError::XmlError(e)),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, XmlDecodeError> where F: FnMut(&mut Self, usize) -> Result<T, XmlDecodeError> {
        let name = try!(self.read_str());
        if let Some(pos) = names.iter().position(|v| *v == &*name) {
            f(self, pos)
        } else {
            Err(XmlDecodeError::InvalidFormat)
        }
    }

    fn read_enum_variant_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, XmlDecodeError> where F: FnMut(&mut Self, usize) -> Result<T, XmlDecodeError> {
    }

    fn read_enum_struct_variant_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_struct_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_tuple<T, F>(&mut self, len: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_tuple_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_tuple_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_option<T, F>(&mut self, f: F) -> Result<T, XmlDecodeError> where F: FnMut(&mut Self, bool) -> Result<T, XmlDecodeError> {
    }

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self, usize) -> Result<T, XmlDecodeError> {
    }

    fn read_seq_elt<T, F>(&mut self, idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_map<T, F>(&mut self, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self, usize) -> Result<T, XmlDecodeError> {
    }

    fn read_map_elt_key<T, F>(&mut self, idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn read_map_elt_val<T, F>(&mut self, idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
    }

    fn error(&mut self, err: &str) -> XmlDecodeError {
        XmlDecodeError::XmlError(XmlError::new_full(0, 0, err.to_string()))
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
