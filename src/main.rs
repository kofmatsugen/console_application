use sprite_studio_converter::convert_to_timeline;

fn main() -> Result<(), Box<std::error::Error>> {
    env_logger::init();
    println!(
        "{:?}",
        convert_to_timeline(
            "amethyst_sandbox/resources",
            "data/sprite_studio/houou.sspj",
        )?
    );

    Ok(())
}
