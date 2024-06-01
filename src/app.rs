use clap::Args;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;

pub struct App<A: Args> {
    args: A,
    is_edit: bool,
}

impl<A: Args> App<A> {
    pub fn new(args: A, is_edit: bool) -> App<A> {
        App { args, is_edit }
    }

    pub fn init(self) -> Result<App<A>, Box<dyn Error>> {
        if let Some(dir) = dirs::config_dir() {
            let conf_dir = dir.join("emoji-commit");
            if !conf_dir.exists() {
                fs::create_dir_all(&conf_dir)?;
            }
            let conf_path = conf_dir.join("config.json");
            if !conf_path.exists() {
                OpenOptions::new().write(true).open(&conf_path)?;
                fs::write(&conf_path, "[]")?;
            }
        }
        Ok(self)
    }

    pub fn run_app(self) {}
}
