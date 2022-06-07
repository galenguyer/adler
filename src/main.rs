use std::path::Path;

use simple_logger::SimpleLogger;
mod module;

fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_local_timestamps()
        .init()
        .unwrap();

    let dirs = std::fs::read_dir("./modules/")
        .unwrap()
        .map(|path| path.unwrap().path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();

    let root_dir = std::env::current_dir().unwrap();

    log::info!("found {} modules", dirs.len());

    for dir in dirs {
        log::debug!("entering dir: {}", dir.display());
        std::env::set_current_dir(&dir).unwrap();
        let module: module::Module =
            serde_yaml::from_str(&std::fs::read_to_string("_module.yaml").unwrap()).unwrap();
        module.apply();
        log::debug!("leaving dir: {}", dir.display());
        std::env::set_current_dir(&root_dir).unwrap();
    }
}
