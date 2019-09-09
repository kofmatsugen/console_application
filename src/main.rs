use sprite_studio_converter::convert_to_timeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!(
        "{:?}",
        convert_to_timeline(
            "amethyst_sandbox/resources",
            "data/sprite_studio/studio_logo/splash1024.sspj",
        )?
    );

    Ok(())
}
