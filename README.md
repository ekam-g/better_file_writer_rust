# What does this package do?

this package make many folders with one command, rather then using a loop and causing an error.

# How do i use it?

```
fn main() {
    let path = "yes/cool/somepath/makefile".to_string();
    // use this one if you want to pass in a &String value
    better_file_maker::make_folders(&path).expect("");
    // use this one if you want to pass in a normal string value
    better_file_maker::drop_make_folders(path).expect("");
}
```
