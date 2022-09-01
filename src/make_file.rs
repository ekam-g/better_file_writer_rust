use std::{fs, io::Error};

pub fn make_dic(path: String) -> Result<(), Error> {
    let directory_error = fs::create_dir(&path);
    return match directory_error {
        Ok(_) => Ok(()),
        Err(e) => {
            let mut success = false;
            let full: String = path.replace("database/", "");
            let where_file = full.split("/");
            let mut location: String = "database/".to_string();
            for i in where_file {
                location = location + i + "/";
                let directory_error = fs::create_dir(&location);
                match directory_error {
                    Ok(_) => {
                        success = true;
                    }
                    Err(_) => {}
                };
            }
            if success {
                Ok(())
            } else {
                Err(e)
            }
        }
    };
}
