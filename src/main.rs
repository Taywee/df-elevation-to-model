extern crate image;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};

fn main() {
    let matches = App::new("df-elevation-to-model")
        .version("0.1")
        .author("Taylor C. Richberger <taywee@gmx.com>")
        .about("Converts a Dwarf Fortress elevation map to a 3D model")
        .arg(Arg::with_name("elevation")
             .short("e")
             .long("elevation")
             .value_name("IMAGE")
             .required(true)
             .help("Takes in an elevation file.")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILE")
             .required(true)
             .help("Output ply file.")
             .takes_value(true))
        .get_matches();
    let img = image::open(matches.value_of("elevation").unwrap()).unwrap().to_rgb();
    let (width, height) = img.dimensions();

    let mut f = File::create(matches.value_of("output").unwrap()).unwrap();
    f.write(format!("ply\n\
        format ascii 1.0\n\
        element vertex {}\n\
        property int x\n\
        property int y\n\
        property int z\n\
        element face {}\n\
        property list uchar int vertex_index\n\
        end_header\n",
        width * height,
        (width - 1) * (height - 1)).as_bytes()
   ).unwrap();

    for (x, y, pixel) in img.enumerate_pixels() {
        let height = match pixel.data {
            // b maxes out at 100 when others hit 75; need to scale by that amount
            [0, 0, b] => b as u32,
            [r, g, b] => (r as u32 + g as u32 + b as u32) * 4 / 9,
        };
        f.write(format!("{} {} {}\n", x, y, height).as_bytes()).unwrap();
    }

    for y in 0..(height - 1) {
        for x in 0..(width - 1) {
            let startIndex = width * y + x;
            f.write(format!("4 {} {} {} {}\n", startIndex, startIndex + 1, startIndex + 1 + width, startIndex + width).as_bytes()).unwrap();
        }
    }
}
