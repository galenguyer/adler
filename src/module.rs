use crate::module::file::FileModule;
use serde::{Deserialize, Serialize};

pub mod file;

#[derive(Serialize, Deserialize, Debug)]
pub enum Module {
    #[serde(rename = "file")]
    File(FileModule),
    #[serde(rename = "directory")]
    Directory,
    #[serde(rename = "package")]
    Package,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModuleType {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

impl Module {
    pub fn apply(&self) {
        match self {
            Module::File(m) => {
                log::debug!("found module: {}", m.name);
                match m.apply() {
                    Ok(_) => log::info!("applied module: {}", m.name),
                    Err(_) => log::error!("failed to apply module: {}", m.name),
                }
            }
            Module::Directory => {
                unimplemented!();
            }
            Module::Package => {
                unimplemented!();
            }
        }
    }
}
