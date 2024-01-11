use assert_cmd::prelude::*; 
use std::process::Command;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::fs::{self, ReadDir};


#[test]
fn test_inputs() -> Result<(), Box<dyn std::error::Error>> {
    // let paths = fs::read_dir("./").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    // let mut test = Command::cargo_bin("drewno_mars")?;
    p1();
    // test.args(&["./tests/input/test1.dm", "-u"]);
    // test.assert()
    //     .success();

    Ok(())
}

fn p1() -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir("./tests/p1").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }

    // let mut test = Command::cargo_bin("drewno_mars")?;

    // loop {
    //     let dm = match paths.next() {
    //         Some(x) => x.unwrap(),
    //         None => break
    //     };
    //     let err = match paths.next() {
    //         Some(x) => x.unwrap(),
    //         None => break
    //     };
    //     let expected = match paths.next() {
    //         Some(x) => x.unwrap(),
    //         None => break
    //     };
    //     test.args(&["./tests/input/test1.dm", "-t"]);
    //     test.assert()
    //         .success();
    // }
    
    chunk_by_test(paths);
    

    Ok(())
}


/// Chunks an array of test files into vectors for each individual test. Each set of test files should contain
/// three filetypes: `.dm`, `.err`, and `.expected`. They do not have to be in order
/// Panics if there aren't all three extensions for one file name.
///
/// # Arguments
/// 
/// * `files` - A `ReadDir` of test files. 
/// 
/// # Examples
///
/// ```
/// let tests = vec![
///     "t1.dm", "t2.err", "t1.expected", "t2.dm", 
///     "t1.expected", "t3.err", "t3.dm", "t3.err", 
///     "t2.expected",
/// ];
/// let tests = chunk_by_test(tests);
///
/// assert_eq!(tests, vec![
///     vec!["t1.dm", "t1.err", "t1.expected"],
///     vec!["t2.dm", "t2.err", "t2.expected"],
///     vec!["t3.dm", "t3.err", "t3.expected"],
/// ]);
/// ```
fn chunk_by_test(files: ReadDir) -> HashMap<String, Test> {
    let mut tests : HashMap<String, Test> = HashMap::new();

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
        match tests.get_mut(&name) {
            Some(x) => {
                match extension {
                    "dm" => x.dm = true,
                    "err" => x.err = true,
                    "expected" => x.expected = true,
                    &_ => continue
                };
                continue;
            },
            None => ()
        };
        let mut temp = Test {
            path: path.clone().parent().unwrap().to_path_buf(),
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
        tests.insert(name.to_string(), temp);
    }

    dbg!(&tests);
    tests
}

#[derive(Debug)]
/// A test
struct Test {
    path: PathBuf,
    dm: bool,
    err: bool,
    expected: bool
}

impl Test {
    fn has_all_files(&self) -> bool {
        self.dm && self.err && self.expected
    }
    // fn files(&self) -> Vec<String> {
    //     self.dm && self.err && self.expected
    // }
}