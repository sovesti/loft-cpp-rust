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

pub enum Bracket {
    LBrace,
    RBrace,
    LCurly,
    RCurly
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
        self.write(&[self.prefix.clone().buf.as_bytes(), b"\"", key, b"\": \"", value, b"\",\n"]);
    }

    pub fn render_line_without_value(self: &mut Self, key: &[u8]) {
        self.write(&[self.prefix.clone().buf.as_bytes(), b"\"", key, b"\":\n"]);
    }

    pub fn render_bracket(self: &mut Self, br: Bracket) {
        let self_prefix = self.prefix.clone();
        match br {
            Bracket::LBrace => self.write(&[self_prefix.buf.as_bytes(), b"[\n"]),
            Bracket::RBrace => self.write(&[self_prefix.buf.as_bytes(), b"]\n"]),
            Bracket::LCurly => self.write(&[self_prefix.buf.as_bytes(), b"{\n"]),
            Bracket::RCurly => self.write(&[self_prefix.buf.as_bytes(), b"}\n"]),
        }
    }
}

