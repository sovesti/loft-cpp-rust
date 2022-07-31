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

use std::{env, fs, path::PathBuf};
use rustop::opts;
use parse_cpp::parse_trees;

pub mod json;
pub mod graph;
pub mod parse_cpp;
pub mod kind;
pub mod get_name;
pub mod index;

fn check_slash(path: &mut String) {
    if !path.ends_with('/') {
        path.push('/');
    }
    *path = path.replace("\\", "/");
}

fn append_includes(mut parse_options: Vec<String>, mut include_dirs: Vec<String>) -> Vec<String> {
    include_dirs.append(env::split_paths(&env::var_os("PATH").unwrap())
    .map(|path| String::from(path.to_str().unwrap())).collect::<Vec<String>>().as_mut());
    for mut path in include_dirs {
        check_slash(&mut path);
        parse_options.push(String::from(String::from("-I") + &path));
    }
    parse_options
}

fn find_code_in_dir(path: &PathBuf) -> Vec<String> {
    let mut result = Vec::new();
    if path.is_dir() {
        let read_path = fs::read_dir(path).unwrap();
        for entry in read_path {
            let entry = entry.unwrap();
            if entry.file_name().to_str().unwrap().contains(".cpp") || entry.file_name().to_str().unwrap().contains(".c") {
                result.push(String::from(entry.path().to_str().unwrap()));
            }
        }
    }
    result
}

fn expand_input(input_files_and_dirs: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for mut path in input_files_and_dirs {
        if PathBuf::from(path.clone()).is_dir() {
            check_slash(&mut path);
            result.append(&mut find_code_in_dir(&PathBuf::from(path)));
        } else {
            path = path.replace("\\", "/");
            result.push(path);
        }
    }
    result
}

fn config() -> (Vec<String>, Vec<String>, String, Vec<String>, bool) {
    let (mut args, _) = opts! {
        opt input_files:Vec<String> = Vec::new(), 
        desc:"Input files. If you put directory here, program will parse all .cpp and .c files there.", multi:true;
        opt output_dir:String=String::from("./"), desc:"Output directory, default is the current.";
        opt include:Vec<String> = Vec::new(), desc:"Include path.", multi:true;
        opt parse_options:Vec<String> = Vec::new(), desc:"Options passed to clang directly.", multi:true;
        opt exclude:Vec<String> = Vec::new(), desc:"Directories from which AST nodes shouldn't be dumped.", multi:true;
        opt system_headers:bool=false, desc:"Dump cursors from system headers";
    }.parse_or_exit();
    check_slash(&mut args.output_dir);
    (expand_input(args.input_files), 
    append_includes(args.parse_options, args.include.clone()), 
    args.output_dir, 
    args.exclude,
    args.system_headers)
}

fn main() {
    let (input_files, parse_options, output_dir, exclude_dirs, system_headers) = config();
    parse_trees(input_files, parse_options, output_dir, exclude_dirs, system_headers);
}
