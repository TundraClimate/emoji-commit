use std::env;
use std::process::Command;

pub fn open_editor(profile: &str) {
    let editor = env::var("EDITOR").unwrap_or("vi".to_string());
    let home = dirs::home_dir();
    if let Some(home) = home {
        Command::new(editor)
            .arg(format!(
                "{}/.config/emoji-commit/profile/{}",
                home.to_str().unwrap_or("$HOME"),
                profile
            ))
            .status()
            .expect("Could not open editor");
    }
}

pub fn git_commit(msg: String) {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .status()
        .expect("Could not create commit");
}
