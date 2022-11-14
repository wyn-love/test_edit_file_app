
use std::{path::PathBuf, io};
use std::fs::{read_dir,rename};
use log::warn;

pub struct OpenDir {
    dir: PathBuf,
    prefix: String,
}


impl OpenDir {
    pub fn new(dir: PathBuf,prefix: String) -> OpenDir {
        OpenDir { dir, prefix }
    }


    pub fn edit_files_name(&mut self) -> io::Result<()> {
        
        if self.dir.is_dir() {
            for entry in read_dir(&self.dir)? {
                let entry = entry?;

                if entry.path().is_file() {
                    match entry.file_name().to_str() {
                        Some(old_file_name) => {
                            let new_file_name = change_file_name(old_file_name,&mut self.prefix);
                            let mut file_path = PathBuf::clone(&self.dir);
                            file_path.push(old_file_name);
                            rename(&file_path,file_path.with_file_name(new_file_name))?;
                        },
                        None => {},
                    };
                } else {
                    warn!("{:?} is not a file",entry.file_name());
                }
            }
        }


        Ok(())
    }
}



fn change_file_name(old_file_name: &str,arr: &mut String) -> String {

    let mut arr = arr.clone();

    let index = match old_file_name.find('.') {
        Some(index) => index,
        None => {
            warn!("{} dont have '.' ",old_file_name);
            return old_file_name.into();
        },
    };

    let last_index = match old_file_name.rfind('.') {
        Some(index) => index,
        None => {
            warn!("{} dont have '.' ",old_file_name);
            return old_file_name.into();
        },
    };

    if index == last_index {
        return old_file_name.into();
    };

    arr.push_str(&old_file_name[..index]);
    arr.push_str(&old_file_name[index+1..]);

    arr

    // for b in old_file_name.as_bytes() {
        
    // }

}

