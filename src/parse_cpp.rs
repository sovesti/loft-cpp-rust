/* 
   Copyright (c) 2022 ArSysOp.

   Licensed under the Apach&e License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
  
       http:  www.apache.org/licenses/LICENSE-2.0
  
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
  
   SPDX-License-Identifier: Apache-2.0
  
   Contributors:
     ArSysOp - initial API and implementation
*/

use crate::json::*;
use clang::*;
use std::{fs::*, io::Write, path::PathBuf};
use crate::graph::*;

const DIAGNOSTICS : bool = true;
const EXCLUDE : bool = true;

fn get_parser<'a> (index: &'a Index, path: PathBuf) -> Parser<'a> {
    index.parser(path)
}

fn get_tu<'a> (parser: &'a Parser) -> TranslationUnit<'a> {
    parser.parse().expect("failed to parse file")
}

enum OutFileKey {
    _AST,
    _CallGraph,
}

impl OutFileKey {
    fn get_key (&self) -> String {
        match self {
            OutFileKey::_AST => String::from("_ast"),
            OutFileKey::_CallGraph => String::from("_call_graph"),
        }
    }
}

fn create_output_file (input_file_path: &str, output_dir: &str, key: OutFileKey) -> File {
    create_dir_all(output_dir.clone()).unwrap();
    let input_file_path_as_path = PathBuf::from(input_file_path);
    let input_file_path_without_extension = PathBuf::from(input_file_path_as_path.file_stem().unwrap().to_str().unwrap());
    let output_file_name = input_file_path_without_extension.file_name().unwrap().to_str().unwrap();
    File::create(String::from(output_dir) + output_file_name + &key.get_key() + ".json").unwrap()
}

fn build_call_graph<'tu>(label: String, ast: AST<'tu>, exclude_dirs: &Vec<String>, system_headers: bool) -> CallGraph<'tu> {
    let mut result = CallGraph::new(label, ast.clone());
    result.take_callable_from_ast(ast);
    for node in result.nodes.clone() {
        result.add_callees(node, exclude_dirs, system_headers);
    }
    result
}

fn get_ast_and_serializer<'tu>(tu: &'tu TranslationUnit, path: &str, output_dir: &str, key: OutFileKey)
 -> (AST<'tu>, JSONSerializer) {
    let ast = AST::new(tu.get_entity());
    let json = JSONSerializer::new(create_output_file(path, output_dir, key));
    (ast, json)
}

fn dump_ast(tu: &TranslationUnit, path: &str, output_dir: &str, exclude_dirs: &Vec<String>, system_headers: bool) {
    let (mut ast, mut json) = get_ast_and_serializer(tu, path, output_dir, OutFileKey::_AST);
    let node: Node = Node::new(tu.get_entity(), &mut ast, true, exclude_dirs, system_headers).0;
    json.render_bracket(Bracket::LCurly);
    json.prefix.expand();
    json = node.serialize(json);
    json.prefix.shrink();
    json.render_bracket(Bracket::RCurly);
    json.writer.flush().unwrap();
}

fn dump_call_graph(tu: &TranslationUnit, path: &str, output_dir: &str, exclude_dirs: &Vec<String>, system_headers: bool) {
    let (ast, mut json) = get_ast_and_serializer(tu, path, output_dir, OutFileKey::_CallGraph);
    let path_as_path = PathBuf::from(path);
    let call_graph = build_call_graph(String::from(path_as_path.file_name().unwrap().to_str().unwrap()), ast, exclude_dirs, system_headers); 
    json = call_graph.serialize(json);
    json.writer.flush().unwrap();
}

pub fn parse_trees(
    input_files: Vec<String>, 
    mut parse_options: Vec<String>, 
    output_dir: String, 
    exclude_dirs: Vec<String>, 
    system_headers: bool) 
{
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, EXCLUDE, DIAGNOSTICS);
    for path in input_files {
        let mut parser = get_parser(&index, PathBuf::from(path.clone()));
        parser.arguments(&mut parse_options);
        let tu = get_tu(&parser);
        dump_ast(&tu, &path, &output_dir, &exclude_dirs, system_headers);
        dump_call_graph(&tu, &path, &output_dir, &exclude_dirs, system_headers);
    }
}

