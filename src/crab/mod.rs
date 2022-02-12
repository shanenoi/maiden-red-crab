pub mod app;
pub mod config;

static APP_CONFIG_DIR: &str = "apps.yml";
static CRAB_CONFIG: &str = "crab.yml";
static BASE_PATH: &str = "/tmp";
static DEFAULT_FILEMOD: u32 = 0o755;

#[allow(dead_code)]
static ERROR_CONFIG: &str = "[ERR] Load the config file: ";

static ERROR_CONFIG_FILE: &str = "[ERR] config file: ";
static ERROR_CREATE_APP_CONFIG_DIR: &str = "[ERR] Can't create app config dir";
static ERROR_FIND_CONFIG_DIR: &str = "[ERR] Can't find config dir";
static PKG_NAME: &str = env!("CARGO_PKG_NAME");

static APP_CONFIG_TEMPLATE: &str = "apps:
  vim.md: google-chrome-stable --app=https://vim.md/
  messenger: google-chrome-stable --app=https://messenger.com
  keep_g: google-chrome-stable --app=https://keep.google.com/
  zalo: google-chrome-stable --app=https://chat.zalo.me
";

static CRAB_CONFIG_TEMPLATE: &str = "
apps_path: <CRAB_CONFIG>/apps.yml
executable_dir_path: <HOME>/.local/bin
";