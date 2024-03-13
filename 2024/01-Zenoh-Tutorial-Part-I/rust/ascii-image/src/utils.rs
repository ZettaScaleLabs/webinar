
extern crate asciify;
use asciify::AsciiBuilder;

use prost::Message;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/demo.rs"));
}

struct ByteStream {
    buf: Vec<u8>
}
impl ByteStream {
    pub fn new() -> ByteStream {
        ByteStream { buf: Vec::new() }
    }
    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }

    pub fn write(&mut self, buf: &[u8]) {
        for b in buf {
            self.buf.push(*b);
        }
    }
}

impl std::io::Write for ByteStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn img2ascii(img_path: &str) -> String {
    let builder = AsciiBuilder::new_from_path(img_path.into());

    let mut byte_stream = ByteStream::new();
    builder.to_stream(&mut byte_stream);
    let v = byte_stream.into_vec();

    if let Ok(s) = String::from_utf8(v) {
        s
    } else { String::from("Failed to create Image!") }
}
impl From<String> for proto::Image {
    fn from(value: String) -> Self {
        proto::Image { format: "ascii".into(), img:  value.into(), height: 0, width: 0 }
    }
}



pub fn img_to_bytes(img: &proto::Image) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(img.encoded_len());
    img.encode(&mut buf).unwrap();
    buf
}

pub fn bytes_to_img(bs: Vec<u8>) -> proto::Image {
    proto::Image::decode(&mut std::io::Cursor::new(bs)).unwrap()
}