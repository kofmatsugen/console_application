use sprite_studio_converter::convert_to_file;

fn main() -> Result<(), Box<std::error::Error>> {
    env_logger::init();
    println!(
        "{:?}",
        convert_to_file(
            "data/sprite_studio/converted",
            "data/sprite_studio/houou.sspj",
        )?
    );

    Ok(())
}
