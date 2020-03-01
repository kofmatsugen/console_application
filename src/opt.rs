use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub config_file: Option<PathBuf>,
    #[structopt(subcommand)]
    pub command: SubCommand,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "subcommands")]
pub enum SubCommand {
    #[structopt(about = "convert spritestudio data")]
    SpriteStudio,
    #[structopt(about = "convert command data")]
    Command,
}
