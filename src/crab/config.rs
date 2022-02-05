use crate::crab::{
    BASE_PATH,
    ERROR_CREATE_APP_CONFIG_DIR,
    ERROR_FIND_CONFIG_DIR,
    PKG_NAME,
    app::AppsConfig,
};
use dirs;
use std::fs;
use std::path::{ PathBuf };

pub struct Config {
    apps_path:           &'static str,
    executable_dir_path: &'static str,
}

pub trait Init {
    fn init(&self) -> PathBuf;
    fn create_apps_config(&self);
    fn load_apps_config(&self);
    fn create_excutable_dir(&self);
    fn load_excutable_dir(&self);
}

pub trait Export {
    fn read_shellrc(&self) -> Vec<String>;
    fn check_shellrc(&self) -> bool;
    fn export(&self);
}

impl Config {
    pub fn new() -> Box<Config> {
        Box::new(Config {
            apps_path: "",
            executable_dir_path: "",
        })
    }
}

impl Init for Config {
    fn init(&self) -> PathBuf {
        let default_config_dir: PathBuf;
        if let Some(config_dir) = dirs::config_dir() {
            default_config_dir = config_dir;
            if let Err(_) = fs::create_dir_all(default_config_dir.join(PKG_NAME)) {
                panic!("{}", ERROR_CREATE_APP_CONFIG_DIR);
            }
        } else {
                panic!("{}", ERROR_FIND_CONFIG_DIR);
        }

        default_config_dir
    }

    // let test_file = "/tmp/test.txt";
    // if !Path::new(test_file).exists() {
    //     let mut file = File::create(test_file).unwrap();
    //     file.write_all("this the strigng".as_bytes()).unwrap();
    //     file.sync_all().unwrap();
    // }
    fn create_apps_config(&self) {}

    fn load_apps_config(&self) {}

    fn create_excutable_dir(&self) {}

    fn load_excutable_dir(&self) {}
}
