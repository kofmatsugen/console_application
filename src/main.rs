mod config;
mod opt;

use config::Config;
use fight_game::{
    id::{
        command::Command,
        pack::{AnimationKey, PackKey},
    },
    paramater::AnimationParam,
    resource::command::CommandList,
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
    env_logger::init();
    let opt = Opt::from_args();

    let config_path = opt
        .config_file
        .unwrap_or(PathBuf::from("./ConsoleConfig.toml"));

    let config = read_config(&config_path)?;

    match &opt.command {
        opt::SubCommand::SpriteStudio => {
            convert_to_timeline::<_, AnimationParam, PackKey, AnimationKey>(
                &config.resource_path,
                &PathBuf::from("data/sprite_studio/sample/sample.sspj"),
            )?;
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
    }

    Ok(())
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
