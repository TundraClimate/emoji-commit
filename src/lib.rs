mod app;
pub use app::App;

pub mod command_processor;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Prefix to use
    prefix: Option<String>,

    /// Commit message
    msg: Option<String>,

    /// Edit ec config
    #[arg(short, long)]
    edit: bool,
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
