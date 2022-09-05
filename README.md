# What does this package do?

this package make many folders with one command, rather then using a loop and causing an error.

# How does it work?

The code loop through every slash and creates a new file and handles the errors, then returns the result.


# How do i use it?

To use a String value

```
fn main() {
    better_file_maker::drop_make_folders("yes/cool/somepath/makefile".to_string()).expect("");
}
```

To use a &String value

```
fn main() {
    better_file_maker::make_folders(&"yes/cool/somepath/makefile".to_string()).expect("");
}
```

# What can cause an error?

1. If the all of the folder exist it will throw an error.
2. If the permissions where denied
3. OS errors
