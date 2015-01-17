#![allow(unstable)]

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

#[derive(Show)]
enum XmlNode {
    Tag {
        name: String,
        attributes: BTreeMap<String, String>,
        children: Vec<XmlNode>
    },
    Text(String)
}

impl std::fmt::String for XmlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            XmlNode::Text(ref s) => f.write_str(&**s),
            XmlNode::Tag { ref name, ref children, ref attributes } => {
                try!(f.write_str("<"));
                try!(f.write_str(&**name));
                for attr in attributes.iter() {
                    try!(f.write_str(" "));
                    try!(f.write_str(&**attr.0));
                    try!(f.write_str("=\""));
                    try!(f.write_str(&**attr.1));
                    try!(f.write_str("\""));
                }
                if children.len() > 0 {
                    try!(f.write_str(">"));
                    for child in children.iter() {
                        try!(child.fmt(f));
                    }
                    try!(f.write_str("</"));
                    try!(f.write_str(&**name));
                    try!(f.write_str(">"));
                } else {
                    try!(f.write_str(" />"));
                }
                Ok(())
            }
        }
    }
}

impl XmlNode {
    fn empty() -> XmlNode {
        XmlNode::Tag {
            name: String::new(),
            attributes: BTreeMap::new(),
            children: Vec::new()
        }
    }

    fn text(&self) -> String {
        match *self {
            XmlNode::Tag { ref children, .. } => children.iter().filter_map(|ref v| match *v {
                &XmlNode::Text(ref s) => Some(&**s),
                _ => None
            }).collect(),
            XmlNode::Text(ref s) => s.clone()
        }
    }

    fn text_rec(&self) -> String {
        let mut result = String::new();
        match *self {
            XmlNode::Tag { ref children, .. } => {
                for node in children.iter() {
                    result.push_str(&*node.text_rec());
                }
            },
            XmlNode::Text(ref s) => result.push_str(&**s)
        }
        result
    }

    fn parse<T>(&self) -> Result<T, XmlDecodeError> {
        match *self {
            XmlNode::Text(ref s) => match s.parse::<T>() {
                Some(v) => Ok(v),
                None => Err(XmlDecodeError::InvalidFormat)
            },
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn from_xml<'a, B: Buffer>(events: &mut Events<'a, B>) -> Result<Vec<XmlNode>, XmlError> {
        let mut stack = vec![XmlNode::empty()];
        for event in *events {
            match event {
                XmlEvent::StartElement { name, attributes, .. } => {
                    stack.push(XmlNode::Tag {
                        name: name.local_name,
                        attributes: attributes.into_iter().map(|a| (a.name.local_name, a.value)).collect(),
                        children: Vec::new()
                    });
                },
                XmlEvent::EndElement{ name } => {
                    let node = stack.pop().unwrap();
                    let last = stack.len() - 1;
                    if let &mut XmlNode::Tag { ref mut children, .. } = &mut stack[last] {
                        children.push(node);
                    }
                },
                XmlEvent::Characters(s) | XmlEvent::CData(s) => {
                    let last = stack.len() - 1;
                    if let &mut XmlNode::Tag { ref mut children, .. } = &mut stack[last] {
                        children.push(XmlNode::Text(s));
                    }
                },

                XmlEvent::EndDocument => break,

                XmlEvent::Error(e) => return Err(e),

                XmlEvent::StartDocument { .. } => continue,
                XmlEvent::ProcessingInstruction { .. } => continue,
                XmlEvent::Whitespace(_) | XmlEvent::Comment(_) => continue,
            }
        }
        match stack.pop() {
            Some(XmlNode::Tag { children, .. }) => Ok(children),
            _ => Ok(Vec::new())
        }
    }
}

impl std::str::FromStr for XmlNode {
    fn from_str(s: &str) -> Option<XmlNode> {
        let mut reader = EventReader::new(s.as_bytes());
        XmlNode::from_xml(&mut reader.events()).ok().and_then(move |mut v| v.pop())
    }
}

enum XmlDecodeError {
    InvalidFormat,
    UnexpectedTag,
    CustomError(String)
}

impl Decoder for XmlNode {
    type Error = XmlDecodeError;
    fn read_nil(&mut self) -> Result<(), XmlDecodeError> {
        Ok(())
    }

    fn read_usize(&mut self) -> Result<usize, XmlDecodeError> {
        self.read_u64().map(|v| v as usize)
    }

    fn read_u64(&mut self) -> Result<u64, XmlDecodeError> {
        self.parse::<u64>()
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
        self.parse::<i64>()
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
        self.parse::<f64>()
    }

    fn read_f32(&mut self) -> Result<f32, XmlDecodeError> {
        self.read_f64().map(|v| v as f32)
    }

    fn read_char(&mut self) -> Result<char, XmlDecodeError> {
        self.parse::<char>()
    }

    fn read_str(&mut self) -> Result<String, XmlDecodeError> {
        match *self {
            XmlNode::Text(ref s) => Ok(s.clone()),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    // <enum-name>...</enum-name>
    fn read_enum<T, F>(&mut self, n: &str, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        match *self {
            XmlNode::Tag { ref name, .. } if &**name = n => f(self),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    // <enum-name><enum-variant-name>...</enum-variant-name></enum-name>
    fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, XmlDecodeError> where F: FnMut(&mut Self, usize) -> Result<T, XmlDecodeError> {
        let name = match *self {
            XmlNode::Text(ref s) => &**s,
            XmlNode::Tag { ref name } => &**name
        };
        match names.iter().position(|v| *v == name) {
            Some(pos) => f(self, pos),
            None => Err(XmlDecodeError::InvalidFormat)
        }
    }

    // <enum-name><enum-variant-name>...</enum-variant-name></enum-name>
    fn read_enum_variant_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        match *self {
            XmlNode::Tag { ref mut children, .. } => f(&mut children[a_idx]),
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, XmlDecodeError> where F: FnMut(&mut Self, usize) -> Result<T, XmlDecodeError> {
        self.read_enum_variant(names, f)
    }

    fn read_enum_struct_variant_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        self.read_struct_field(f_name, f_idx, f)
    }

    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        match *self {
            XmlNode::Tag { ref name, .. } if &**name == s_name => f(self),
            XmlNode::Text(_) => Err(XmlDecodeError::UnexpectedTag),
            _ => Err(XmlDecodeError::InvalidFormat)
        }
    }

    fn read_struct_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        match *self {
            XmlNode::Tag { ref children, .. } => match children.iter_mut().find(|ref c| match *c {
                XmlNode::Tag { ref name } => &**name == f_name,
                _ => false
            }) {
                Some(c) => f(c),
                None => Err(XmlDecodeError::InvalidFormat)
            },
            _ => Err(XmlDecodeError::UnexpectedTag)
        }
    }

    fn read_tuple<T, F>(&mut self, len: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        match *self {
            XmlNode::Tag { ref children, .. } if children.len() == len => f(self),
            _ => Err(XmlDecodeError::UnexpectedTag),
        }
    }

    fn read_tuple_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        match *self {
            XmlNode::Tag { ref mut children, .. } => f(children[a_idx]),
            _ => Err(XmlDecodeError::UnexpectedTag),
        }
    }

    fn read_tuple_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        self.read_struct(s_name, len, f)
    }

    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, XmlDecodeError> where F: FnOnce(&mut Self) -> Result<T, XmlDecodeError> {
        self.read_enum_variant_arg(a_idx, f)
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
        XmlDecodeError::CustomError(err.to_string())
    }

}


#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use rustc_serialize::{Encoder, Decoder, Encodable, Decodable};

    use super::{XmlNode, XmlDecoder};

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
    fn test_parse() {
        let xml = XML_STRUCT.parse::<XmlNode>().unwrap();
        println!("{}", xml.text_rec());
        println!("{:?}", xml);
        panic!("{}", xml);
    }

    //#[test]
    //fn test_decode() {
        //let mut reader = BufReader::new(XML_STRUCT.as_bytes());
        //let mut decoder = XmlDecoder::new(reader);
        //assert_eq!(SomeStruct {
            //someint: 123,
            //somestr: "a string".to_string(),
            //somevec: vec![1, 2, 3],
            //someopt: Some(100)
        //}, Decodable::decode(&mut decoder).unwrap());
    //}
}
