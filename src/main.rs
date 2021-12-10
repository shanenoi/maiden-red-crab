use std::{
    collections::BTreeMap,
    fs::File,
    io::Write,
    path::{ Path, PathBuf },
    os::unix::fs::PermissionsExt
};

type AppsConfig = BTreeMap<String, BTreeMap<String, String>>;

static DEFAULT_CONFIG: &'static str = "default.yml";
static BASE_PATH: &'static str = "/tmp";
static ERROR_CONFIG: &'static str = "[ERR] Load the config file: ";
static ERROR_CONFIG_FILE: &'static str = "[ERR] config file: ";

#[derive(Debug)]
pub struct App {
    pub name: String,
    pub content: String,
}

impl App {
    pub fn new(config: &BTreeMap<String, String>) -> Vec<Box<App>> {
        let mut result = Vec::new();
        for (key, value) in &*config {
            result.push(Box::new(App {
                name: key.to_string(),
                content: value.to_string(),
            }));
        }
        return result;
    }

    pub fn abs_path(&self) -> PathBuf {
        Path::new(BASE_PATH).join(&self.name)
    }

    pub fn save(&self) {
        let mut file = File::create(self.abs_path()).unwrap();
        file.write_all(self.content.as_bytes()).unwrap();
        file.sync_all().unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let file_config = File::open(DEFAULT_CONFIG);

    match file_config {
        Ok(config) => {
            let deserialized_map: Result<AppsConfig, serde_yaml::Error> =
                serde_yaml::from_reader(&config);
            match deserialized_map {
                Ok(map) => {
                    match map.get("apps") {
                        Some(value) => {
                            let apps = App::new(value);
                            for app in apps {
                                app.save();
                            }
                        }
                        None => {}
                    }
                    Ok(())
                }
                Err(err) => panic!("{} {}", ERROR_CONFIG, err),
            }
        }
        Err(err) => panic!("{} {}", ERROR_CONFIG_FILE, err),
    }
}
