mod config;
mod convert;
mod opt;

use amethyst_sprite_studio::splash::SplashTranslation;
use config::Config;
use fight_game::{
    id::command::Command, paramater::FightTranslation, resource::command::CommandList,
};
use opt::Opt;
use sprite_studio_converter::convert_to_timeline;
use std::{
    fs,
    io::{BufReader, Read},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();

    let config_path = opt
        .config_file
        .unwrap_or(PathBuf::from("./ConsoleConfig.toml"));

    let config = read_config(&config_path)?;

    init_log(&opt.verbose);

    env_logger::init();
    match &opt.command {
        opt::SubCommand::SpriteStudio => {
            for file in config.convert_fight_animations {
                log::info!("fight convert start: {:?} ", file);
                match convert_to_timeline::<_, FightTranslation>(&config.resource_path, &file) {
                    Ok(_) => {
                        log::info!("fight convert success: {:?}", file);
                    }
                    Err(err) => {
                        log::error!("fight convert fail: {:?}", file);
                        log::error!("error: {}", err);
                    }
                }
            }
            for file in config.convert_splash_animations {
                log::info!("splash convert start: {:?} ", file);
                match convert_to_timeline::<_, SplashTranslation>(&config.resource_path, &file) {
                    Ok(_) => {
                        log::info!("splash convert success: {:?}", file);
                    }
                    Err(err) => {
                        log::error!("splash convert fail: {:?}", file);
                        log::error!("error: {}", err);
                    }
                }
            }
            for file in config.convert_test_animations {
                log::info!("test convert start: {:?} ", file);
                match convert_to_timeline::<_, convert::test_convert::TestFile>(
                    &config.resource_path,
                    &file,
                ) {
                    Ok(_) => {
                        log::info!("test convert success: {:?}", file);
                    }
                    Err(err) => {
                        log::error!("test convert fail: {:?}", file);
                        log::error!("error: {}", err);
                    }
                }
            }
        }
        opt::SubCommand::Command => {
            let dir = config.resource_path.join("command");
            std::fs::create_dir_all(&dir)?;
            let dir = dir.join("basic.com.ron");

            let mut list = CommandList::new();
            list.add_command(Command::A, "pA[1]")?;
            list.add_command(Command::B, "pB[1]")?;
            list.add_command(Command::C, "pC[1]")?;
            list.add_command(Command::D, "pD[1]")?;

            data_to_file(list, dir)?;
        }
        opt::SubCommand::Test => {}
    }

    Ok(())
}

fn init_log(verbose: &opt::Verbose) {
    let env_name = "RUST_LOG";
    match verbose {
        opt::Verbose::Info => std::env::set_var(env_name, "info"),
        opt::Verbose::Error => std::env::set_var(env_name, "error"),
        opt::Verbose::Debug => std::env::set_var(env_name, "debug"),
        opt::Verbose::Trace => std::env::set_var(env_name, "trace"),
        opt::Verbose::Warning => std::env::set_var(env_name, "warn"),
    }
}

fn read_config<P: AsRef<Path>>(path: &P) -> Result<Config, failure::Error> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path).map(|f| BufReader::new(f))?;

    fr.read_to_string(&mut file_content)?;

    Ok(toml::from_str(&file_content)?)
}

fn data_to_file<S, P>(data: S, path: P) -> std::result::Result<(), failure::Error>
where
    S: serde::Serialize,
    P: AsRef<std::path::Path> + std::fmt::Debug,
{
    let config = ron::ser::PrettyConfig {
        depth_limit: std::usize::MAX,
        new_line: "\n".into(),
        indentor: "\t".into(),
        separate_tuple_members: false,
        enumerate_arrays: true,
    };
    log::info!("save: {:?}", path);
    let string = ron::ser::to_string_pretty(&data, config)?;
    let file = std::fs::File::create(path)?;
    let mut buff = BufWriter::new(file);
    buff.write(string.as_bytes())?;
    Ok(())
}

// fn file_to_data<T, P>(path: P) -> std::result::Result<T, failure::Error>
// where
//     T: serde::de::DeserializeOwned,
//     P: AsRef<std::path::Path> + std::fmt::Debug,
// {
//     log::info!("load: {:?}", path);
//     let file = std::fs::File::open(path)?;
//     let buff = BufReader::new(file);
//     let data = ron::de::from_reader(buff)?;
//     Ok(data)
// }
