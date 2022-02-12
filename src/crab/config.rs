use crate::crab::{
    APP_CONFIG_DIR,
    APP_CONFIG_TEMPLATE,
    CRAB_CONFIG,
    CRAB_CONFIG_TEMPLATE,
    ERROR_CONFIG_FILE,
    ERROR_CREATE_APP_CONFIG_DIR,
    ERROR_FIND_CONFIG_DIR,
    PKG_NAME,
    app::AppsConfig,
};

use dirs;
use regex::Regex;
use serde::{ Deserialize, Serialize, de::Error, };
use std::io::{ Write };
use std::path::{ PathBuf };
use std::fmt::Debug;
use std::{ collections::BTreeMap, fs, fs::File, };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    apps_path:           &'static str,
    executable_dir_path: &'static str,
}

pub trait Init {
    fn init(&self) -> PathBuf;
    fn create_apps_config(&self) -> PathBuf;
    fn load_apps_config(&self) -> Result<AppsConfig, serde_yaml::Error>;
    fn create_crab_config(&self) -> PathBuf;
    fn load_crab_config(&mut self);
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

    fn update(&mut self, other: &'static BTreeMap<String, String>) {
        let app_path = other.get(&"app_path".to_string());
        let executable_dir_path = other.get(&"executable_dir_path".to_string());

        match app_path {
            Some(path) => self.apps_path = path.as_str(),
            None => (),
        }

        match executable_dir_path {
            Some(path) => self.executable_dir_path = path.as_str(),
            None => (),
        }
    }

    fn init_file(filepath: &PathBuf, content: &[u8]) {
        if !filepath.exists() {
            let mut file = File::create(filepath).unwrap();
            file.write_all(content).unwrap();
            file.sync_all().unwrap();
        }
    }

    fn read_yml<T: for<'de> serde::Deserialize<'de>>(&self, filepath: &PathBuf) -> Result<T, serde_yaml::Error> {
        match File::open(filepath) {
            Ok(file) => serde_yaml::from_reader(file),
            Err(err) => Err(serde_yaml::Error::custom(format!("{} {:?}", ERROR_CONFIG_FILE, err))),
        }
    }
}

impl Init for Config {
    fn init(&self) -> PathBuf {
        let default_config_dir: PathBuf;

        if let Some(config_dir) = dirs::config_dir() {
            default_config_dir = config_dir.join(PKG_NAME);
            if let Err(_) = fs::create_dir_all(&default_config_dir) {
                panic!("{}", ERROR_CREATE_APP_CONFIG_DIR);
            }
        } else {
                panic!("{}", ERROR_FIND_CONFIG_DIR);
        }

        default_config_dir
    }

    fn create_apps_config(&self) -> PathBuf {
        let config = self.init().join(APP_CONFIG_DIR);
        Config::init_file(&config, APP_CONFIG_TEMPLATE.as_bytes());
        config
    }

    fn create_crab_config(&self) -> PathBuf {
        let init_ = self.init();
        let config = init_.join(CRAB_CONFIG);

        let crab_config = Regex::new(r"(<CRAB_CONFIG>)").unwrap();
        let home = Regex::new(r"(<HOME>)").unwrap();
        let crab_config_path = (&init_).to_str().unwrap();
        let home_path = dirs::home_dir().unwrap();

        Config::init_file(
            &config,
            home.replace_all(
                &crab_config.replace_all(CRAB_CONFIG_TEMPLATE, crab_config_path),
                home_path.to_str().unwrap()
            ).as_bytes()
        );

        config
    }

    fn load_apps_config(&self) -> Result<AppsConfig, serde_yaml::Error> {
        self.read_yml::<AppsConfig>(&self.create_apps_config())
    }

    fn load_crab_config(&mut self) {
        if let Ok(config) = self.read_yml::<BTreeMap<String, String>>(&self.create_apps_config()) {
            self.update(&config);
        } else {
            panic!("{}", ERROR_CONFIG_FILE);
        }
    }
}