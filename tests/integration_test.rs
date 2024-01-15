use assert_cmd::prelude::*; 
use std::process::{Command, Output};
use std::iter::zip;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::fs::{self, ReadDir};

#[test]
fn test_inputs() {
    p1();

}

fn p1() {
    let tests = &chunk_by_test(fs::read_dir("./tests/p1").unwrap());

    for t in tests {
        let mut test = Command::cargo_bin("drewno_mars").unwrap();
        let input_file = format!("{}/{}.dm", t.directory.to_str().unwrap(), *t.name);
        let correct_file_path = format!("{}/{}.expected", t.directory.to_str().unwrap(), *t.name);
        let err_file_path = format!("{}/{}.err", t.directory.to_str().unwrap(), t.name);

        let correct_input = match std::fs::read_to_string(correct_file_path) {
            Ok(v) => v,
            Err(_) => panic!("Unable to read given input file.")
        };
        let correct_errs = match std::fs::read_to_string(err_file_path) {
            Ok(v) => v,
            Err(_) => panic!("Unable to read given input file.")
        };
        test.args(&[input_file.as_str(), "-t"]);
        let output = test.assert().success();
        let Output { 
            stdout: output,
            stderr: err,
            ..
        } = output.get_output();
        
        let err = String::from_utf8(err.to_vec()).unwrap();
        let output = String::from_utf8(output.to_vec()).unwrap();

        for (correct, out) in zip(correct_input.lines(), output.lines()) {
            assert_eq!(correct, out);
        }

        for (correct, out) in zip(correct_errs.lines(), err.lines()) {
            assert_eq!(correct, out);
        }
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