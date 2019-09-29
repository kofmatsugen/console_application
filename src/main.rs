use sprite_studio_converter::convert_to_timeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    convert_to_timeline(
        "amethyst_sandbox/resources",
        "data/sprite_studio/studio_logo/splash1024.sspj",
    )?;
    convert_to_timeline(
        "amethyst_sandbox/resources",
        "data/sprite_studio/houou/houou.sspj",
    )?;
    convert_to_timeline(
        "amethyst_sandbox/resources",
        "data/sprite_studio/template/character_template1.sspj",
    )?;
    convert_to_timeline(
        "amethyst_sandbox/resources",
        "data/sprite_studio/sample/sample.sspj",
    )?;
    Ok(())
}
