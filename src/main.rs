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
        opt::SubCommand::Spirv => {
            let fragment_dir =
                fs::canonicalize(Path::new("./amethyst-sprite-studio/src/shader/fragment"))?;
            let fragment_out_dir = fs::canonicalize(Path::new(
                "./amethyst-sprite-studio/src/shader/compiled/fragment",
            ))?;
            let compiler_path = fs::canonicalize(Path::new("./spirv/bin/glslangValidator.exe"))?;
            for path in std::fs::read_dir(&fragment_dir)? {
                let path = path?.path();
                let compile_path = format!("{}", path.display());
                let out_path = format!(
                    "{}",
                    fragment_out_dir
                        .join(path.file_name().unwrap())
                        .with_extension("frag.spv")
                        .display()
                );
                log::info!("compile: {}", compile_path);
                let output = std::process::Command::new(&compiler_path)
                    .args(&["-H", &compile_path, "-o", &out_path])
                    .output()?;
                log::info!("{}", String::from_utf8(output.stdout)?);
                log::info!("compile finish: {}", out_path);
                log::info!("compile status: {}", output.status);
            }

            let vertex_dir =
                fs::canonicalize(Path::new("./amethyst-sprite-studio/src/shader/vertex"))?;
            let vertex_out_dir = fs::canonicalize(Path::new(
                "./amethyst-sprite-studio/src/shader/compiled/vertex",
            ))?;
            for path in std::fs::read_dir(&vertex_dir)? {
                let path = path?.path();
                let compile_path = format!("{}", path.display());
                let out_path = format!(
                    "{}",
                    vertex_out_dir
                        .join(path.file_name().unwrap())
                        .with_extension("vert.spv")
                        .display()
                );
                log::info!("compile: {}", compile_path);
                let output = std::process::Command::new(&compiler_path)
                    .args(&["-H", &compile_path, "-o", &out_path])
                    .output()?;
                log::info!("{}", String::from_utf8(output.stdout)?);
                log::info!("compile finish: {}", out_path);
                log::info!("compile status: {}", output.status);
            }
        }
        opt::SubCommand::Analyze => {
            for path in visit_dirs(&config.resource_path)?
                .into_iter()
                .filter_map(|f| {
                    let file_name = format!("{}", f.display());
                    if file_name.contains(".anim.ron") {
                        Some(PathBuf::from(file_name))
                    } else {
                        None
                    }
                })
            {
                match ron::de::from_reader(std::fs::File::open(&path)?) {
                    Ok(data) => log::info!(
                        "{:?}: {:?}",
                        path,
                        fight_game::types::analyze::SkillInfomation::make_info(
                            &data,
                            fight_game::id::pack::PackKey::Base,
                            fight_game::id::pack::AnimationKey::Punch
                        )
                    ),
                    Err(_) => {}
                };
            }
        }
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

// one possible implementation of walking a directory only visiting files
fn visit_dirs<P: AsRef<Path>>(dir: &P) -> std::io::Result<Vec<PathBuf>> {
    let dir = dir.as_ref();
    let mut files = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut nest_files = visit_dirs(&path)?;
                files.append(&mut nest_files);
            } else {
                files.push(entry.path());
            }
        }
    }
    Ok(files)
}
