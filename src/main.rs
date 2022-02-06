mod crab;
use crab::app::{ App, AppsConfig };
use crab::config::{ Config, Init };
use std::fs::File;

static DEFAULT_CONFIG: &str = "default.yml";
static ERROR_CONFIG: &str = "[ERR] Load the config file: ";
static ERROR_CONFIG_FILE: &str = "[ERR] config file: ";

fn main() -> std::io::Result<()> {
    let file_config = File::open(DEFAULT_CONFIG);
    let config_ = Config::new();
    println!("{:?}", config_.init());
    config_.create_apps_config();

    match file_config {
        Ok(config) => {
            let deserialized_map: Result<AppsConfig, serde_yaml::Error> =
                serde_yaml::from_reader(&config);
            match deserialized_map {
                Ok(map) => {
                    if let Some(value) = map.get("apps") {
                        let apps = App::new(value);
                        for app in apps {
                            app.save();
                            println!("installed to {:?}", app.abs_path());
                        }
                    }
                    Ok(())
                }
                Err(err) => panic!("{} {}", ERROR_CONFIG, err),
            }
        }
        Err(err) => panic!("{} {}", ERROR_CONFIG_FILE, err),
    }
}
