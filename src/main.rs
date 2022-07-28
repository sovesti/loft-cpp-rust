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

fn fill_excludes(exclude: &mut Vec<String>, include_paths: Vec<String>) {
    if *exclude == vec![String::from("-")] {
        exclude.pop();
    } else {
        if exclude.is_empty() {
            for mut path in include_paths {
                check_slash(&mut path);
                path.pop();
                match path.rfind("/") {
                    Some(last_slash) => {
                        path.drain((last_slash - 1)..);
                    },
                    None => {}
                }
                exclude.push(path);
            }
        }
    }
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
            result.push(path);
        }
    }
    result
}

fn config() -> (Vec<String>, Vec<String>, String, Vec<String>) {
    let (mut args, _) = opts! {
        opt input_files:Vec<String> = Vec::new(), 
        desc:"Input files. If you put directory here, program will parse all .cpp and .c files there.", multi:true;
        opt output_dir:String=String::from("./"), desc:"Output directory, default is the current.";
        opt include:Vec<String> = Vec::new(), desc:"Include path.", multi:true;
        opt parse_options:Vec<String> = Vec::new(), desc:"Options passed to clang directly.", multi:true;
        opt exclude:Vec<String> = Vec::new(), desc:"Directories from which AST nodes shouldn't be traversed. 
        By default, program will try to put included libraries there which may not work correctly. 
        Set \"-\" to exclude nothing.", multi:true;
    }.parse_or_exit();
    fill_excludes(&mut args.exclude, 
    append_includes(args.parse_options.clone(), args.include.clone()));
    check_slash(&mut args.output_dir);
    (expand_input(args.input_files), 
    append_includes(args.parse_options, args.include.clone()), 
    args.output_dir, 
    args.exclude)
}

fn main() {
    let (input_files, parse_options, output_dir, exclude_dirs) = config();
    parse_trees(input_files, parse_options, output_dir, exclude_dirs);
}
