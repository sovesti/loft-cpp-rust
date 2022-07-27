use std::env;
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

fn config() -> (Vec<String>, Vec<String>, String) {
    let (mut args, _) = opts! {
        opt input_files:Vec<String> = Vec::new(), desc:"Input files.", multi:true;
        opt output_dir:String=String::from("./"), desc:"Output directory, default is the current";
        opt include:Vec<String> = Vec::new(), desc:"Include path", multi:true;
        opt parse_options:Vec<String> = Vec::new(), desc:"Options passed to clang directly", multi:true;
    }.parse_or_exit();
    check_slash(&mut args.output_dir);
    (args.input_files, append_includes(args.parse_options, args.include), args.output_dir)
}

fn main() {
    let (input_files, parse_options, output_dir) = config();
    parse_trees(input_files, parse_options, output_dir);
}
