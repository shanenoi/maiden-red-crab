mod crab;
use crab::app::{ App, AppsConfig };
use std::fs::File;

static DEFAULT_CONFIG: &'static str = "default.yml";
static ERROR_CONFIG: &'static str = "[ERR] Load the config file: ";
static ERROR_CONFIG_FILE: &'static str = "[ERR] config file: ";

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
                                println!("installed to {:?}", app.abs_path());
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
