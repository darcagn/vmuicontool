// VMU Icon Tool
// Convert a 32x32 pixel icon with max 16 colors into a VMU icon byte array for C
// (c) 2023 darc

use image::{imageops::FilterType, Pixel, Rgba};
use std::{env, path::Path, process::exit};

// These four constants control formatting settings:
// Indent all lines by this number of spaces...
const INDENT_ALL: usize = 0;
// Indent bytes inside blocks by this number of spaces...
const INDENT_BLOCK: usize = 4;
// This number of palette bytes will be printed per line...
const PAL_BYTES_PER_LINE: usize = 8;
// This number of data bytes will be printed per line...
const DATA_BYTES_PER_LINE: usize = 8;

fn print_usage() {
    eprintln!("vmu icon tool - (c) 2023 darc");
    eprintln!("takes a 32x32 pixel icon with max 16 colors and outputs a byte array for C");
    eprintln!("Usage: vmuicontool <imagefile>");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        print_usage();
        exit(1);
    }

    let inputpath = Path::new(&args[0]);

    let src_image = {
        if let Ok(src_image) = image::open(inputpath) {
            src_image.resize(32, 32, FilterType::Lanczos3).into_rgba8()
        } else {
            print_usage();
            eprintln!("Could not open file {:?}. Quitting...", &inputpath);
            exit(1);
        }
    };

    let mut colorvec: Vec<u16> = vec![];

    let pixel2vmucolor = |pixel: Rgba<u8>| -> u16 {
        u16::from_le_bytes([
            (pixel[1] & 0b1111_0000) + (pixel[2] >> 4),
            (pixel[3] & 0b1111_0000) + (pixel[0] >> 4),
        ])
    };

    for &pixel in src_image.pixels() {
        if !colorvec.contains(&pixel2vmucolor(pixel)) {
            colorvec.push(pixel2vmucolor(pixel));
        }
    }

    if colorvec.len() > 16 {
        eprintln!(
            "Number of colors is too large! Your image needs 16 colors or fewer. Quitting..."
        );
        exit(1);
    }

    colorvec.resize(16, 0);

    print!(
        "{}static const unsigned short vmu_icon_pal[16] = {{ ",
        " ".repeat(INDENT_ALL)
    );
    for (i, item) in colorvec.iter().enumerate() {
        if i % PAL_BYTES_PER_LINE == 0 {
            print!("\n{}", " ".repeat(INDENT_ALL + INDENT_BLOCK));
        }
        print!("0x{:X}, ", item);
    }
    print!("\n{}}};\n\n", " ".repeat(INDENT_ALL));

    let mut offsvec: Vec<u8> = vec![];
    for pixel in src_image.pixels() {
        if let Some(x) = colorvec
            .iter()
            .position(|&x| x == pixel2vmucolor(pixel.to_rgba()))
        {
            offsvec.push(x as u8);
        } else {
            eprintln!("Error during image conversion. Quitting...");
            exit(1);
        }
    }

    let mut datavec = vec![];
    for i in 0..512 {
        let byte: u8 = (offsvec[i * 2] << 4) + offsvec[(i * 2) + 1];
        datavec.push(byte);
    }

    print!(
        "{}static const unsigned char vmu_icon_data[512] = {{ ",
        " ".repeat(INDENT_ALL)
    );
    for (i, byte) in datavec.iter().enumerate() {
        if i % DATA_BYTES_PER_LINE == 0 {
            print!("\n{}", " ".repeat(INDENT_ALL + INDENT_BLOCK));
        }
        print!("0x{:02X}, ", byte);
    }
    print!("\n{}}};\n", " ".repeat(INDENT_ALL));
}
