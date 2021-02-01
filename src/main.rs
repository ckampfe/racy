use racy::Options;

use std::fs::File;
use std::io::prelude::*;
use std::{error::Error, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    // options.stl_path
    // let file = std::fs::File::open("/Users/Clark/code/nom_stl/fixtures/Root_Vase.stl").unwrap();
    // let file = std::fs::File::open("/home/clark/code/personal/Moon.stl").unwrap();
    // let file = std::fs::File::open("/Users/clark/Downloads/Moon.stl").unwrap();
    let file =
        std::fs::File::open("/Users/clark/code/nom_stl/fixtures/MOON_PRISM_POWER.stl").unwrap();
    // let file = std::fs::File::open("/Users/clark/Downloads/rpi3-top_rev03.stl").unwrap();

    let mut bytes = BufReader::new(file);

    let mut f = File::create("prism_power.png")?;

    let options = Options {
        image_format: image::ImageFormat::Png,
        ..Default::default()
    };

    let stl = nom_stl::parse_stl(&mut bytes).unwrap();
    let rendered = racy::render(&stl, &options).unwrap();
    f.write_all(&rendered).unwrap();

    Ok(())
}
