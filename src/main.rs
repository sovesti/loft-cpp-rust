use crate::json::JSONSerializer;
use std::fs::File;
use clang;

pub mod json;
pub mod graph;

fn main() {
    let mut file = File::create("test.txt").expect("failed to create file");
    let mut json = JSONSerializer::new(file);
    let mut data = || {json.render_line(b"key", b"value")};
}
