//! Make files easier

fn main() {
    let path = "yes/cool/somepath/makefile".to_string();
    better_file_maker::make_folders(&path).expect("");
    better_file_maker::drop_make_folders(path).expect("");
}
pub mod lib;
