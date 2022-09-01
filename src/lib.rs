/// Loops and creates directories where you want them
/// this will create multiple directories even if some/path does not exist.
/// if the folder exist it will move on and ignore it.
/// If all of the folder creation functions fail then it will return an error.
/// # Examples
///
/// ```
/// make_folders("some/path/test".to_string()).expect("fail");
///
///```
///
use std::{fs, io::Error};

pub fn make_folders(path: String) -> Result<(), Error> {
    let directory_error = fs::create_dir(&path);
    return match directory_error {
        Ok(_) => Ok(()),
        Err(mut error) => {
            let mut success = false;
            let where_file = path.split("/");
            let mut location: String = "".to_string();
            for i in where_file {
                location = location + i + "/";
                let directory_error = fs::create_dir(&location);
                match directory_error {
                    Ok(_) => {
                        success = true;
                    }
                    Err(e) => {
                        error = e;
                    }
                };
            }
            if success {
                Ok(())
            } else {
                Err(error)
            }
        }
    };
}
