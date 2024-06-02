use crate::command_processor;
use clap::Args;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;

pub struct App<A: Args> {
    args: A,
    is_edit: bool,
}

const DEFAULT_CONF_FILE: &str = r#"
[
    {
        "key": "feat",
        "prefix": ":sparkles:feat: "
    },
    {
        "key": "fix",
        "prefix": ":zap:fix: "
    },
    {
        "key": "refactor",
        "prefix": ":recycle:refactor: "
    },
    {
        "key": "docs",
        "prefix": ":memo:docs: "
    },
    {
        "key": "wip",
        "prefix": ":fire:wip: "
    },
    {
        "key": "chore",
        "prefix": ":pencil2:chore: "
    }
]
"#;

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
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&conf_path)?;
                fs::write(&conf_path, DEFAULT_CONF_FILE)?;
            }
        }
        Ok(self)
    }

    pub fn run_app(self) {
        let eflag = self.is_edit;
        if eflag {
            command_processor::open_editor();
        } else {
        }
    }
}
