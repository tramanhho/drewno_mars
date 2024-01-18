use assert_cmd::prelude::*; 
use std::process::{Command, Output};
use std::iter::zip;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::fs::{self, ReadDir};

#[test]
fn project_1() {
    test(1);
}

#[test]
fn project_3() {
    test(3);
}

#[test]
fn project_4() {
    test(4);
}

#[test]
fn project_5() {
    test(5);
}

#[test]
fn project_6() {
    test(6);
}

#[test]
fn project_7() {
    test(7);
}

fn test(project: u8) {
    let tests : &Vec::<Test> = &chunk_by_test(fs::read_dir(format!("./tests/p{}", project)).unwrap());

    for t in tests {
        let mut command = Command::cargo_bin("drewno_mars").unwrap();
        let directory = t.directory.to_str().unwrap();

        let input_file    = format!("{}/{}.dm",       directory, *t.name);
        let correct_input = format!("{}/{}.expected", directory, *t.name);
        let correct_errs  = format!("{}/{}.err",      directory, *t.name);

        let correct_input = match std::fs::read_to_string(correct_input) {
            Ok(v) => v,
            Err(_) => panic!("Unable to read given input file.")
        };
        let correct_errs = match std::fs::read_to_string(correct_errs) {
            Ok(v) => v,
            Err(_) => panic!("Unable to read given input file.")
        };
        command.args(&[input_file.as_str(), option_from_project(project)]);
        let output = command.assert().success();
        let Output { 
            stdout: output,
            stderr: err,
            ..
        } = output.get_output();
        
        let err = String::from_utf8(err.to_vec()).unwrap();
        let output = String::from_utf8(output.to_vec()).unwrap();
        
        for (correct, out) in zip(correct_input.lines(), output.lines()) {
            assert_eq!(correct.replace("\t", "    ").trim(), out.replace("\t", "    ").trim());
        }

        for (correct, out) in zip(correct_errs.lines(), err.lines()) {
            assert_eq!(correct.trim(), out.trim());
        }
    }
}

fn option_from_project(project: u8) -> &'static str {
    match project {
        1 => "-t",
        2 => "-p",
        3 => "-u",
        4 => "-n",
        5 => "-c",
        6 => "-a",
        7 => "-o",
        _ => "-t"
    }
}

/// Chunks an array of test files into vectors for each individual test. Each set of test files should contain
/// three filetypes: `.dm`, `.err`, and `.expected`. They do not have to be in order
/// Panics if there aren't all three extensions for one file name.
///
/// # Arguments
/// 
/// * `files` - A `ReadDir` of test files. 
/// 
/// ```
fn chunk_by_test(files: ReadDir) -> Vec<Test> {
    let mut init_tests : HashMap<String, InitTest> = HashMap::new();
    let mut tests : Vec<Test> = Vec::new();

    for f in files {
        let path = f.unwrap().path();
        let name = match path.clone().file_stem().and_then(OsStr::to_str) {
            Some(x) => x.to_owned(),
            None => continue
        };
        let extension = match path.extension().and_then(OsStr::to_str) {
            Some(x) => x,
            None => continue
        };
        match init_tests.get_mut(&name) {
            Some(x) => {
                match extension {
                    "dm" => x.dm = true,
                    "err" => x.err = true,
                    "expected" => x.expected = true,
                    &_ => continue
                };
                if x.has_all_files() {
                    let test = Test {
                        name: Box::new(name),
                        directory: path.parent().unwrap().to_path_buf()
                    };
                    tests.push(test);
                }
                continue;
            },
            None => ()
        };
        let mut temp = InitTest {
            dm: false,
            err: false,
            expected : false
        };
        match extension {
            "dm" => temp.dm = true,
            "err" => temp.err = true,
            "expected" => temp.expected = true,
            &_ => continue
        };
        init_tests.insert(name.to_string(), temp);
    }

    tests
}

#[derive(Debug)]
/// A test
struct InitTest {
    dm: bool,
    err: bool,
    expected: bool
}

#[derive(Debug)]
/// A test
struct Test {
    name: Box<String>,
    directory: PathBuf
}

impl InitTest {
    fn has_all_files(&self) -> bool {
        self.dm && self.err && self.expected
    }
}