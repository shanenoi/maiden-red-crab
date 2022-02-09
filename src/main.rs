mod crab;
use crab::app::{ App };
use crab::config::{ Config, Init };

static ERROR_CONFIG: &str = "[ERR] Load the config file: ";
static ERROR_CONFIG_FILE: &str = "[ERR] config file: ";

fn main() -> std::io::Result<()> {
    let config_ = Config::new();
    println!("{:?}", config_.init());
    config_.create_apps_config();
    config_.create_crab_config();

    if let Ok(config) = config_.load_apps_config() {
        if let Some(value) = config.get("apps") {
            let apps = App::new(value);
            for app in apps {
                app.save();
                println!("installed to {:?}", app.abs_path());
            }
        } else {
            panic!("{}", ERROR_CONFIG)
        }
    } else {
        panic!("{}", ERROR_CONFIG_FILE)
    }
    Ok(())
}