use crate::crab::{
    BASE_PATH,
    DEFAULT_FILEMOD,
};
use std::{
    collections::BTreeMap,
    io::Write,
    path::{ Path, PathBuf },
    fs::{ File, Permissions, set_permissions },
    os::unix::fs::PermissionsExt,
};

pub type AppsConfig = BTreeMap<String, BTreeMap<String, String>>;


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
        set_permissions(
            self.abs_path(),
            Permissions::from_mode(DEFAULT_FILEMOD)
        ).unwrap();
        self 
    }

    pub fn save(&self) -> &App {
        let mut file = File::create(self.abs_path()).unwrap();
        file.write_all(self.content.as_bytes()).unwrap();
        file.sync_all().unwrap();
        self.update_filemode()
    }
}
