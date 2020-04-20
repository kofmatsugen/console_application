use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub config_file: Option<PathBuf>,
    #[structopt(short = "V", long = "verbose", default_value = "info")]
    pub verbose: Verbose,
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
    #[structopt(about = "test amethyst")]
    Test,
}

#[derive(StructOpt, Debug)]
pub enum Verbose {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl std::str::FromStr for Verbose {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "error" => Ok(Verbose::Error),
            "warning" => Ok(Verbose::Warning),
            "info" => Ok(Verbose::Info),
            "debug" => Ok(Verbose::Debug),
            "trace" => Ok(Verbose::Trace),
            _ => Err(format!("unsupported verbose: {}", s)),
        }
    }
}
