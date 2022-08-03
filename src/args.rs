use clap::{ArgAction, Parser};

#[derive(Debug, Clone, Parser)]
#[clap(name=env!("CARGO_PKG_NAME"), version=env!("CARGO_PKG_VERSION"), about=env!("CARGO_PKG_DESCRIPTION"), ignore_errors=true)]
pub struct Args {
    #[clap(long, short, action=ArgAction::SetTrue)]
    pub init: Option<bool>,

    #[clap(long = "dest-dir", short)]
    pub dest_dir: Option<String>,

    #[clap(long, action=ArgAction::SetTrue)]
    pub test: Option<bool>,

    #[clap(long, short, action=ArgAction::SetTrue)]
    pub verbose: Option<bool>,
}
