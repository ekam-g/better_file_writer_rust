//! Make files easier

fn main() {
    lib::make_folders("test/yes/cool/yes/farf".to_string()).expect("");
}

/// Loops and creates directories where you want them
///
/// this will create multiple directories even if some/path does not exist.
/// # Examples
///
/// ```
/// make_dic("some/path/test".to_string()).expect("fail");
///
///```
///
pub mod lib;
