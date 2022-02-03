use crate::crab::{
    app::AppsConfig,
    BASE_PATH,
};

pub struct Config {
    apps_path:           &'static str,
    executable_dir_path: &'static str,
}

impl Config {
    pub fn new() -> Box<Config> {
        Box::new(Config {
            apps_path: "",
            executable_dir_path: "",
        })
    }

    pub fn source(&self) {}

    pub fn execute_path(&self) {}

    pub fn export_shellrc(&self) {}

    fn init_config(&self) {}
}
