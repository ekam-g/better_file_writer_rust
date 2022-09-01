//! Make files easier

fn main() {
    lib::make_folders("test/yes/cool/yes/farf".to_string()).expect("");
}
pub mod lib;
