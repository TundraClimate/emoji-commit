use std::process::Command;

pub fn open_editor(profile: &str) -> std::io::Result<()> {
    let editor = option_env!("EDITOR").unwrap_or("vi");

    let Some(home) = dirs::home_dir() else {
        eprintln!("ec terminated: Not supported OS");
        std::process::exit(1)
    };

    Command::new(editor)
        .arg(format!(
            "{}/.config/emoji-commit/profile/{}",
            home.to_str().unwrap_or("$HOME"),
            profile
        ))
        .status()?;

    Ok(())
}

pub fn git_commit(msg: String) {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .status()
        .expect("Could not create commit");
}
