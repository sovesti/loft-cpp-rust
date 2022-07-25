use std::io::BufWriter;
use std::io::Write;
use std::fs::File;

const INDENT: usize = 2;
const FILL: char = ' ';

#[derive(Clone)]
pub struct Prefix {
    sz: usize,
    buf: String,
}

impl Prefix {
    fn new() -> Prefix {
        Prefix {
            sz: 0,
            buf: String::from("")
        }
    }

    pub fn expand(self: &mut Self) {
        self.sz += INDENT;
        for _ in 0..INDENT {
            self.buf.push(FILL);
        }
    }

    pub fn shrink(self: &mut Self) {
        assert!(self.sz >= INDENT);
        self.sz -= INDENT;
        self.buf.truncate(self.sz);
    }
}

// thanks to @burubum from StackOverFlow 
fn replace<T>(source: &[T], from: &[T], to: &[T]) -> Vec<T>
where
    T: Clone + PartialEq
{
    let mut result = source.to_vec();
    let from_len = from.len();
    let to_len = to.len();

    let mut i = 0;
    while i + from_len <= result.len() {
        if result[i..].starts_with(from) {
            result.splice(i..i + from_len, to.iter().cloned());
            i += to_len;
        } else {
            i += 1;
        }
    }

    result
}

pub enum Bracket {
    LBrace,
    RBrace,
    LCurly,
    RCurly,
}

pub struct JSONSerializer {
    pub prefix: Prefix,
    pub writer: BufWriter<File>
}

impl JSONSerializer {
    pub fn new(out: File) -> JSONSerializer {
        JSONSerializer {
            prefix: Prefix::new(),
            writer: BufWriter::new(out)
        }
    }

    fn write(self: &mut Self, bufs: &[&[u8]]) {
        for buf in bufs {
            self.writer.write(buf).unwrap();
        }
    }

    pub fn render_line(self: &mut Self, key: &[u8], value: &[u8]) {
        let value_with_escaped_quotes = replace(value, b"\"", b"\\\"");
        self.write(&[b"\n", self.prefix.clone().buf.as_bytes(), b"\"", key, b"\": \"", value_with_escaped_quotes.as_slice(), b"\","]);
    }

    pub fn render_line_without_value(self: &mut Self, key: &[u8]) {
        self.write(&[b"\n", self.prefix.clone().buf.as_bytes(), b"\"", key, b"\":"]);
    }

    pub fn render_comma(self: &mut Self) {
        self.write(&[b","]);
    }

    pub fn render_bracket(self: &mut Self, br: Bracket) {
        let self_prefix = self.prefix.clone();
        match br {
            Bracket::LBrace => self.write(&[b"\n", self_prefix.buf.as_bytes(), b"["]),
            Bracket::RBrace => self.write(&[b"\n", self_prefix.buf.as_bytes(), b"]"]),
            Bracket::LCurly => self.write(&[b"\n", self_prefix.buf.as_bytes(), b"{"]),
            Bracket::RCurly => self.write(&[b"\n", self_prefix.buf.as_bytes(), b"}"]),
        }
    }
}

