use make_file::make_dic;

pub mod files;
mod make_file;
fn main() {
    make_dic("test/cool".to_string()).expect("");
}
