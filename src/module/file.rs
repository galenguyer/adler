use crate::module::ModuleType;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileModule {
    pub name: String,
    #[serde(rename = "type")]
    pub module_type: ModuleType,
    pub path: String,
}

impl FileModule {
    pub fn apply(&self) -> Result<(), ()> {
        match self.module_type {
            ModuleType::System => {
                unimplemented!();
            }
            ModuleType::User => {
                if !Path::new(&self.path).exists() {
                    log::debug!("ack, {} doesn't exist", self.path);
                    return Err(());
                }

                let dest = Path::new(dirs::home_dir().unwrap().join(&self.path).to_str().unwrap())
                    .to_owned();
                log::debug!("dest: {}", dest.display());

                let source_mtime = filetime::FileTime::from_last_modification_time(
                    &std::fs::metadata(&self.path).unwrap(),
                );

                if !dest.exists() {
                    log::debug!("dest {} doesn't exist, creating", dest.display());
                    fs::copy(&self.path, &dest).unwrap();
                    filetime::set_file_mtime(&dest, source_mtime).unwrap();
                } else if dest.exists() && !dest.is_file() {
                    log::debug!("ack, {} is not a file", dest.display());
                    return Err(());
                } else {
                    let dest_mtime = filetime::FileTime::from_last_modification_time(
                        &std::fs::metadata(&dest).unwrap(),
                    );

                    if dest_mtime > source_mtime {
                        log::debug!("dest {} is newer than {}", dest.display(), self.path);
                        return Err(());
                    } else if dest_mtime == source_mtime {
                        log::debug!("dest {} is the same as {}", dest.display(), self.path);
                    } else {
                        log::debug!("src {} is newer than {}", self.path, dest.display());
                        fs::copy(&self.path, &dest).unwrap();
                        filetime::set_file_mtime(&dest, source_mtime).unwrap();
                    }
                }
            }
        }
        Ok(())
    }
}
