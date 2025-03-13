use clap::Parser;
use std::error::Error;

mod app;
pub use app::App;

pub mod cmd;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Prefix to use
    prefix: Option<String>,

    /// Commit message
    msg: Option<String>,

    #[arg(short, long)]
    /// Edit ec config
    edit: bool,

    #[arg(short = 'S', long)]
    /// Set profile
    set_profile: Option<String>,

    #[arg(short = 'D', long)]
    /// Delete profile
    delete_profile: Option<String>,

    #[arg(short = 'L', long)]
    // Show profile list
    list_profile: bool,

    #[arg(short = 'l')]
    // Show available tags
    list_tags: bool,
}

impl Args {
    pub fn prefix(&self) -> Option<String> {
        self.prefix.clone()
    }

    pub fn msg(&self) -> Option<String> {
        self.msg.clone()
    }

    pub fn edit(&self) -> bool {
        self.edit
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let edit = args.edit();
    let app = App::new(args, edit);
    app.init()?.run_app();

    Ok(())
}
