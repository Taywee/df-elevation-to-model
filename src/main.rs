extern crate byteorder;
extern crate image;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt};

fn main() {
    let matches = App::new("df-elevation-to-model")
        .version("0.2")
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
        format binary_big_endian 1.0\n\
        element vertex {}\n\
        property uint16 x\n\
        property uint16 y\n\
        property uint16 z\n\
        element face {}\n\
        property list uint8 uint32 vertex_index\n\
        end_header\n",
        width * height,
        (width - 1) * (height - 1)).as_bytes()
   ).unwrap();

    for (x, y, pixel) in img.enumerate_pixels() {
        let height = match pixel.data {
            // b maxes out at 100 when others hit 75; need to scale by that amount
            [0, 0, b] => b as u16,
            [r, g, b] => ((r as u32 + g as u32 + b as u32) * 4 / 9) as u16,
        };
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(x as u16).unwrap();
        wtr.write_u16::<BigEndian>(y as u16).unwrap();
        wtr.write_u16::<BigEndian>(height).unwrap();
        f.write(&wtr).unwrap();
    }

    for y in 0..(height - 1) {
        for x in 0..(width - 1) {
            let startIndex = width * y + x;
            let mut wtr = vec![];
            wtr.write_u8(4).unwrap();
            wtr.write_u32::<BigEndian>(startIndex).unwrap();
            wtr.write_u32::<BigEndian>(startIndex + 1).unwrap();
            wtr.write_u32::<BigEndian>(startIndex + 1 + width).unwrap();
            wtr.write_u32::<BigEndian>(startIndex + width).unwrap();
            f.write(&wtr).unwrap();
        }
    }
}
