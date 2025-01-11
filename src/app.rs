use crate::command_processor;
use crate::Args;
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::sync::LazyLock;

pub struct App {
    args: Args,
    is_edit: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PrefMap {
    key: String,
    prefix: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Config {
    profile: String,
}

const DEFAULT_CONF_FILE: LazyLock<Value> = LazyLock::new(|| {
    json!(Config {
        profile: "default".to_string()
    })
});

const DEFAULT_PROFILE: LazyLock<Value> = LazyLock::new(|| {
    json!([
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
    ])
});

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
                fs::write(&conf_path, DEFAULT_CONF_FILE.to_string())?;
            }
            let prof_path = conf_dir.join("profile");
            if !prof_path.exists() {
                fs::create_dir_all(&prof_path)?;
            }
            let default_prof_path = prof_path.join("default.json");
            if !default_prof_path.exists() {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&default_prof_path)?;
                fs::write(&default_prof_path, DEFAULT_PROFILE.to_string())?;
            }
        }
        Ok(self)
    }

    pub fn run_app(self) {
        if self.is_edit {
            let Some(conf_dir) = dirs::config_dir() else {
                eprintln!("Cannot find config directory");
                return;
            };
            let conf_dir = conf_dir.join("emoji-commit");
            let Ok(Config { profile }) = serde_json::from_reader(BufReader::new(
                File::open(conf_dir.join("config.json")).unwrap(),
            )) else {
                eprintln!("Could not open config file");
                return;
            };
            command_processor::open_editor(&format!("{}.json", &profile));
        } else if let Some(new_profile) = self.args.set_profile {
            let Some(conf_dir) = dirs::config_dir() else {
                eprintln!("Cannot find config directory");
                return;
            };
            let conf_dir = conf_dir.join("emoji-commit");
            let Ok(Config { profile: _ }) = serde_json::from_reader(BufReader::new(
                File::open(conf_dir.join("config.json")).unwrap(),
            )) else {
                eprintln!("Could not open config file");
                return;
            };
            let new_conf = json!(Config {
                profile: new_profile.clone()
            });
            fs::write(conf_dir.join("config.json"), new_conf.to_string()).unwrap();
            let prof_dir_path = conf_dir.join("profile");
            let new_prof_path = prof_dir_path.join(format!("{}.json", new_profile));
            if !new_prof_path.exists() {
                fs::copy(prof_dir_path.join("default.json"), new_prof_path).unwrap();
                println!("Create new profile: {}", new_profile);
            } else {
                println!("Switch profile: {}", new_profile);
            }
        } else if let Some(delete_prof) = self.args.delete_profile {
            let Some(conf_dir) = dirs::config_dir() else {
                eprintln!("Cannot find config directory");
                return;
            };
            let conf_dir = conf_dir.join("emoji-commit");
            let Ok(Config { profile }) = serde_json::from_reader(BufReader::new(
                File::open(conf_dir.join("config.json")).unwrap(),
            )) else {
                eprintln!("Could not open config file");
                return;
            };
            if profile == delete_prof {
                let new_conf = json!(Config {
                    profile: String::from("default"),
                });
                fs::write(conf_dir.join("config.json"), new_conf.to_string()).unwrap();
                println!("Switch profile: default");
            }

            let profile_dir = conf_dir.join("profile");
            let target_profile_path = profile_dir.join(format!("{}.json", delete_prof));
            if target_profile_path.exists() {
                fs::remove_file(target_profile_path).unwrap();
                println!("Profile removed: {}", delete_prof);
            } else {
                eprintln!("Profile \"{}\" is Not found.", delete_prof);
            }
        } else if self.args.list_profile {
            let Some(conf_dir) = dirs::config_dir() else {
                eprintln!("Cannot find config directory");
                return;
            };
            let prof_dir = conf_dir.join("emoji-commit").join("profile");
            let Ok(Config { profile }) = serde_json::from_reader(BufReader::new(
                File::open(conf_dir.join("emoji-commit").join("config.json")).unwrap(),
            )) else {
                eprintln!("Could not open config file");
                return;
            };
            fs::read_dir(prof_dir).unwrap().for_each(|entry| {
                if let Ok(entry) = entry {
                    if let Some(stem) = entry.path().file_stem() {
                        if let Some(stem) = stem.to_str() {
                            print!("{stem}");
                            if profile == stem {
                                print!(" (Current)");
                            }
                            println!();
                        }
                    }
                }
            });
        } else if self.args.list_tags {
        } else {
            let args = (self.args.prefix, self.args.msg);
            if let (Some(key), Some(msg)) = args {
                if let Some(conf_path) = dirs::config_dir() {
                    let conf_path = conf_path.join("emoji-commit");
                    let conf_file_path = conf_path.join("config.json");
                    let config: Config = serde_json::from_reader(BufReader::new(
                        File::open(&conf_file_path).unwrap(),
                    ))
                    .expect("Could not open config");
                    let mut current_profile = config.profile.as_str();
                    let profile_dir_path = conf_path.join("profile");
                    if !profile_dir_path
                        .join(format!("{}.json", current_profile))
                        .exists()
                    {
                        eprintln!(
                            "Could not find {current_profile} profile: using default profile."
                        );
                        current_profile = "default";
                    }
                    let json: Vec<PrefMap> = serde_json::from_reader(BufReader::new(
                        File::open(profile_dir_path.join(format!("{}.json", current_profile)))
                            .unwrap(),
                    ))
                    .expect("Could not open profile");
                    let map = json.into_iter().find(|p| p.key == key);
                    if let Some(map) = map {
                        let msg = format!("{}{}", &map.prefix, msg);
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
