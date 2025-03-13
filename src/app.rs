use crate::cmd;
use crate::Args;
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
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

static DEFAULT_CONF_FILE: LazyLock<Value> = LazyLock::new(|| {
    json!(Config {
        profile: "default".to_string()
    })
});

static DEFAULT_PROFILE: LazyLock<Value> = LazyLock::new(|| {
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

fn config_dir_path() -> std::path::PathBuf {
    dirs::config_dir().unwrap_or_else(|| {
        eprintln!("ec terminated: Not supported OS");
        std::process::exit(1);
    })
}

impl App {
    pub fn new(args: Args, is_edit: bool) -> App {
        App { args, is_edit }
    }

    pub fn init(self) -> Result<App, Box<dyn Error>> {
        let conf_dir = config_dir_path().join("emoji-commit");

        if !conf_dir.exists() {
            fs::create_dir_all(&conf_dir)?;
        }

        let conf_path = conf_dir.join("config.json");

        if !conf_path.exists() {
            fs::write(&conf_path, DEFAULT_CONF_FILE.to_string())?;
        }

        let prof_path = conf_dir.join("profile");

        if !prof_path.exists() {
            fs::create_dir_all(&prof_path)?;
        }

        let default_prof_path = prof_path.join("default.json");

        if !default_prof_path.exists() {
            fs::write(&default_prof_path, DEFAULT_PROFILE.to_string())?;
        }

        Ok(self)
    }

    fn edit_config(&self, path: &Path) {
        let Ok(Config { profile }) =
            serde_json::from_reader(BufReader::new(File::open(path).unwrap()))
        else {
            eprintln!("ec terminated: invalid config");
            eprintln!("cause: invalid profile name");
            return;
        };

        if let Err(e) = cmd::open_editor(&format!("{}.json", &profile)) {
            eprintln!("ec terminated: couldn't open of editor");
            eprintln!("cause: {}", e.kind());
        }
    }

    fn set_profile(&self, new_profile: &str, conf_dir: &Path) {
        let new_conf = json!(Config {
            profile: new_profile.to_string()
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
    }

    fn delete_profile(&self, delete_prof: &str, conf_dir: &Path) {
        let Ok(Config { profile }) = serde_json::from_reader(BufReader::new(
            File::open(conf_dir.join("config.json")).unwrap(),
        )) else {
            eprintln!("ec terminated: invalid config");
            eprintln!("cause: invalid profile name");
            return;
        };

        if profile == delete_prof {
            let new_conf = json!(Config {
                profile: String::from("default"),
            });
            fs::write(conf_dir.join("config.json"), new_conf.to_string()).unwrap();
            println!("Switch profile: default");
        }

        let target_profile_path = conf_dir
            .join("profile")
            .join(format!("{}.json", delete_prof));

        if target_profile_path.exists() {
            fs::remove_file(target_profile_path).unwrap();
            println!("Profile removed: {}", delete_prof);
        } else {
            eprintln!("Profile \"{}\" is Not found.", delete_prof);
        }
    }

    fn list_profile(&self, conf_dir: &Path) {
        let prof_dir = conf_dir.join("profile");

        let Ok(Config { profile }) = serde_json::from_reader(BufReader::new(
            File::open(conf_dir.join("config.json")).unwrap(),
        )) else {
            eprintln!("ec terminated: invalid config");
            return;
        };

        fs::read_dir(prof_dir).unwrap().for_each(|entry| {
            if let Some(stem) = entry
                .ok()
                .and_then(|e| e.path().file_stem().map(|s| s.to_owned()))
                .map(|e| e.to_string_lossy().to_string())
            {
                print!("{stem}");
                if profile == stem {
                    print!(" (Current)");
                }
                println!();
            }
        });
    }

    fn list_tags(&self, conf_dir: &Path) {
        let Ok(Config { profile }) = serde_json::from_reader(BufReader::new(
            File::open(conf_dir.join("config.json")).unwrap(),
        )) else {
            eprintln!("ec terminated: invalid config");
            eprintln!("cause: invalid profile name");
            return;
        };

        let profile_path = conf_dir.join("profile").join(format!("{}.json", &profile));

        let Ok(prefs): Result<Vec<PrefMap>, _> =
            serde_json::from_reader(BufReader::new(File::open(profile_path).unwrap()))
        else {
            eprintln!("ec terminated: invalid profile content");
            return;
        };

        prefs
            .into_iter()
            .map(|p| p.key)
            .for_each(|p| println!("{p}"));
    }

    fn create_commit(&self, key: &str, msg: &str, conf_path: &Path) {
        let conf_file_path = conf_path.join("config.json");
        let Ok(Config { profile }) =
            serde_json::from_reader(BufReader::new(File::open(&conf_file_path).unwrap()))
        else {
            eprintln!("ec terminated: invalid config");
            eprintln!("cause: invalid profile name");
            return;
        };
        let mut current_profile = profile.as_str();
        let profile_dir_path = conf_path.join("profile");

        if !profile_dir_path
            .join(format!("{}.json", current_profile))
            .exists()
        {
            eprintln!("profile not found: using to default");
            current_profile = "default";
        }

        let Ok(json): Result<Vec<PrefMap>, serde_json::Error> =
            serde_json::from_reader(BufReader::new(
                File::open(profile_dir_path.join(format!("{}.json", current_profile))).unwrap(),
            ))
        else {
            eprintln!("ec terminated: invalid profile content");
            return;
        };

        let map = json.into_iter().find(|p| p.key == *key);

        if let Some(map) = map {
            let msg = format!("{}{}", &map.prefix, msg);
            cmd::git_commit(msg);
        } else {
            eprintln!("ec terminated: \"{}\" is invalid argument", key);
        }
    }

    pub fn run_app(self) {
        let conf_dir = config_dir_path().join("emoji-commit");

        if self.is_edit {
            let path = conf_dir.join("config.json");

            self.edit_config(&path);
            return;
        }

        if let Some(ref new_profile) = self.args.set_profile {
            self.set_profile(new_profile, &conf_dir);
        }

        if let Some(ref delete_prof) = self.args.delete_profile {
            self.delete_profile(delete_prof, &conf_dir);
        }

        if self.args.list_profile {
            self.list_profile(&conf_dir);
        }

        if self.args.list_tags {
            self.list_tags(&conf_dir);
        }

        if let (Some(key), Some(msg)) = (&self.args.prefix, &self.args.msg) {
            self.create_commit(key, msg, &conf_dir);
        }
    }
}
