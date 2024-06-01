use clap::Parser;
use ec::{App, Args};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let edit = args.edit();
    let app = App::new(args, edit);
    app.init()?.run_app();

    Ok(())
}
