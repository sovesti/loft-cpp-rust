use crate::graph;
use clang::*;
use std::path::PathBuf;
use std::fs::File;

const DIAGNOSTICS : bool = true;
const EXCLUDE : bool = true;

// pub fn get_tu<'a> (path: PathBuf) -> (Clang, Index<'a>, TranslationUnit<'a>, PathBuf) {
//     let clang = Clang::new().expect("failed to create Clang instance");
//     let index = Index::new(&clang, EXCLUDE, DIAGNOSTICS);
//     let parser = index.parser(path);
//     let tu = parser.parse().expect("failed to parse file");
//     let path_to_cache = r".\cache\cache.ast";
//     tu.save(path_to_cache).expect("failed to write to cache");
//     (clang, index, tu, PathBuf::from(path_to_cache))
// }

