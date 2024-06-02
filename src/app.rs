use crate::command_processor;
use crate::Args;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;

pub struct App {
    args: Args,
    is_edit: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PrefMap {
    key: String,
    prefix: String,
}

const DEFAULT_CONF_FILE: &str = r#"
[
    {
        "key": "feat",
        "prefix": "✨feat: "
    },
    {
        "key": "fix",
        "prefix": "⚡️fix: "
    },
    {
        "key": "refactor",
        "prefix": "♻️refactor: "
    },
    {
        "key": "docs",
        "prefix": "📝docs: "
    },
    {
        "key": "wip",
        "prefix": "🔥wip: "
    },
    {
        "key": "chore",
        "prefix": "✏️chore: "
    }
]
"#;

impl App {
    pub fn new(args: Args, is_edit: bool) -> App {
        App { args, is_edit }
    }

    pub fn init(self) -> Result<App, Box<dyn Error>> {
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
            let args = (self.args.prefix, self.args.msg);
            if let (Some(key), Some(msg)) = args {
                if let Some(conf_path) = dirs::config_dir() {
                    let conf_path = conf_path.join("emoji-commit").join("config.json");
                    let json: Vec<PrefMap> =
                        serde_json::from_reader(BufReader::new(File::open(conf_path).unwrap()))
                            .expect("Could not open config");
                    let msgs: Vec<_> = json.into_iter().filter(|p| p.key == key).collect();
                    if msgs.len() != 0 {
                        let msg = format!("{}{}", &msgs[0].prefix, msg);
                        command_processor::git_commit(msg);
                    } else {
                        eprintln!("Could not create commit: invalid arguments \"{}\"", key);
                    }
                }
            } else {
                eprintln!("Could not create commit: invalid arguments");
            }
        }
    }
}
