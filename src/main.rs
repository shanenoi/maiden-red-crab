use std::{
    collections::BTreeMap,
    fs::{ File, Permissions, set_permissions },
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{ Path, PathBuf },
};

type AppsConfig = BTreeMap<String, BTreeMap<String, String>>;

static BASE_PATH: &'static str = "/tmp";
static DEFAULT_CONFIG: &'static str = "default.yml";
static DEFAULT_FILEMOD: u32 = 0o755;
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

    fn update_filemode(&self) -> &App {
        set_permissions(self.abs_path(), Permissions::from_mode(DEFAULT_FILEMOD)).unwrap();
        self 
    }

    pub fn save(&self) -> &App {
        let mut file = File::create(self.abs_path()).unwrap();
        file.write_all(self.content.as_bytes()).unwrap();
        file.sync_all().unwrap();
        self
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
                                app.save().update_filemode();
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
