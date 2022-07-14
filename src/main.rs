use crate::json::JSONSerializer;
use std::{fs::File, io::Write, collections::HashMap};
use clang::*;
use graph::*;
// use parse_cpp::get_tu;
use std::path::PathBuf;
use crate::get_name::*;
use index::collect_entities;

pub mod json;
pub mod graph;
pub mod parse_cpp;
pub mod kind;
pub mod get_name;
pub mod index;


const DIAGNOSTICS : bool = false;
const EXCLUDE : bool = false;

fn main() {
    let file = File::create(r".\test.json").unwrap();
    let mut json = JSONSerializer::new(file);
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, EXCLUDE, DIAGNOSTICS);
    let parser = index.parser(PathBuf::from(r"C:\work\loft-cpp\src\main.cpp"));
    let tu = parser.parse().unwrap();
    for entity in tu.get_entity().get_children() {   
        let set = collect_entities(entity);
        for el in set {
            println!("{}", el);
        }
    }
    let ast = AST::new(tu.get_entity());
    // for entity in ast.nodes {
    //     println!("{}", entity.get_name().get_name());
    // }
    // let mut registry = HashMap::new();
    // let mut nodes = Vec::<Node>::new();
    // for entity in ast.nodes {
    //     let node = Node::new(entity);
    //     json = Node::new(entity).serialize(json);
    //     nodes.push(node);
    // }
    let node = Node::new(tu.get_entity());
    json = node.serialize(json);
    json.writer.flush().unwrap();
    // parse_cpp::construct_graph(file, output);
}
