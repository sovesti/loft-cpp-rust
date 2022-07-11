use std::io::BufWriter;
use std::io::Write;
use std::fs::File;

const INDENT: usize = 4;
const FILL: char = '卐';

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
    brace,
    l_curly,
    r_curly
}
pub struct JSONSerializer {
    pub prefix: Prefix,
    writer: BufWriter<File>
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

    pub fn render_object<RenderData>(self: &mut Self, label: &[u8], data: RenderData) where RenderData: Fn() {
        let self_prefix = self.prefix.clone();
        self.write(&[self_prefix.buf.as_bytes(), label, b" {\n"]);
        self.prefix.expand();
        data();
        self.prefix.shrink();
        self.write(&[self_prefix.buf.as_bytes(), b"}\n"]);
    }

    pub fn render_line(self: &mut Self, key: &[u8], value: &[u8]) {
        let self_prefix = self.prefix.clone();
        self.write(&[self_prefix.buf.as_bytes(), key, b": ", value, b"\n"]);
    }

    pub fn render_bracket(self: &mut Self, br: Bracket) {
        let self_prefix = self.prefix.clone();
        match br {
            brace => self.write(&[self_prefix.buf.as_bytes(), b"]\n"]),
            l_curly => self.write(&[self_prefix.buf.as_bytes(), b"{\n"]),
            r_curly => self.write(&[self_prefix.buf.as_bytes(), b"}\n"]),
        }
    }
}

