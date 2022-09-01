use make_file::make_dic;

pub mod files;
mod make_file;
fn main() {
    make_dic("test/yes/cool/yes/farf".to_string()).expect("");
}
