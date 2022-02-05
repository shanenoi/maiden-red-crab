pub mod app;
pub mod config;

// static DEFAULT_CONFIG: &'static str = "default.yml";
static BASE_PATH: &str = "/tmp";
static DEFAULT_FILEMOD: u32 = 0o755;
static ERROR_CONFIG: &str = "[ERR] Load the config file: ";
static ERROR_CONFIG_FILE: &str = "[ERR] config file: ";
static ERROR_CREATE_APP_CONFIG_DIR: &str = "[ERR] Can't create app config dir";
static ERROR_FIND_CONFIG_DIR: &str = "[ERR] Can't find config dir";
static PKG_NAME: &str = env!("CARGO_PKG_NAME");

static CONFIG_TEMPLATE: &str = "
apps:
  vim.md: google-chrome-stable --app=https://vim.md/
  messenger: google-chrome-stable --app=https://messenger.com
  keep_g: google-chrome-stable --app=https://keep.google.com/
  zalo: google-chrome-stable --app=https://chat.zalo.me
";
