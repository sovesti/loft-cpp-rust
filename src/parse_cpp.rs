/* 
   Copyright (c) 2022 ArSysOp.

   Licensed under the Apache License, Version 2.0 (the "License");
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

use crate::json::JSONSerializer;
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

fn create_output_file (input_file_path: String, output_dir: String) -> File {
    create_dir_all(output_dir.clone()).unwrap();
    let input_file_path_as_path = PathBuf::from(input_file_path);
    let input_file_path_without_extension = PathBuf::from(input_file_path_as_path.file_stem().unwrap().to_str().unwrap());
    let output_file_name = input_file_path_without_extension.file_name().unwrap().to_str().unwrap();
    File::create(output_dir + output_file_name + ".json").unwrap()
}

pub fn parse_trees (input_files: Vec<String>, mut parse_options: Vec<String>, output_dir: String, exclude_dirs: Vec<String>) {
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, EXCLUDE, DIAGNOSTICS);
    for path in input_files {
        let mut parser = get_parser(&index, PathBuf::from(path.clone()));
        parser.arguments(&mut parse_options);
        let tu = get_tu(&parser);
        let out = create_output_file(path, output_dir.clone());
        let mut json = JSONSerializer::new(out);
        let mut ast = AST::new(tu.get_entity(), exclude_dirs.clone());
        let node = Node::new(tu.get_entity(), &mut ast).0;
        json = node.serialize(json);
        json.writer.flush().unwrap();
    }
}

