//! Make files easier

fn main() {
    let path: &str = "yes/cool/somepath/makefile";
    better_file_maker::make_folders(path).expect("");
}
pub mod lib;
