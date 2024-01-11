// extern crate lit;

// #[cfg(test)]
// mod tests {
//     use std::env;
//     use std::env::consts;
//     use std::path::PathBuf;

//     fn bin_dir() -> PathBuf {
//         env::current_exe()
//             .ok()
//             .map(|mut path| {
//                 path.pop();
//                 path.pop();
//                 path
//             })
//             .unwrap()
//     }


//     fn p1() -> Result<(), Box<dyn std::error::Error>> {
    
//         Ok(())
//     }

//     #[test]
//     fn lit() {
//         lit::run::tests(
//             lit::event_handler::Default::new(),
//             |config| {
//             config.add_search_path("./tests/p1");
//             config.add_extension("dm");
//         })
//         .expect("Lit tests failed");
//     }
// }