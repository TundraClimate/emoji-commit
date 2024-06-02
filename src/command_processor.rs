use std::env;
use std::process::Command;

pub fn open_editor() {
    let editor = env::var("EDITOR").unwrap_or("vi".to_string());
    let home = dirs::home_dir();
    if let Some(home) = home {
        Command::new(editor)
            .arg(format!(
                "{}/.config/emoji-commit/config.json",
                home.to_str().unwrap_or("$HOME")
            ))
            .status()
            .expect("Could not open editor");
    }
}
