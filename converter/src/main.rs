use std::{error::Error, fs};

use binrw::{BinWrite, io::Cursor};
use converter::{BaFile, Section};
use video_rs::{Locator, Decoder, Frame};



fn main() -> Result<(), Box<dyn Error>>{
    video_rs::init()?;

    let source = Locator::Path(
        "./assets/badapplesmall.mp4"
            .parse()
            .unwrap(),
    );

    let mut decoder = Decoder::new(&source)?;

    let (source_w, source_h) = decoder.size();
    let source_framerate=  decoder.frame_rate().floor() as u8;

    println!("Processing {}x{} frames...", source_w, source_h);

    let mut bafile = BaFile{
        frame_width: source_w as u8,
        frame_height: source_h as u8,
        frame_rate: source_framerate,
        frame_count: 0,
        frames: Vec::new()
    };

    for frame in decoder.decode_iter() {
        if let Ok((_,frame)) = frame {
            //println!("Processing frame {}...", framecount);
            let baframe = encode_frame(bafile.frame_width, bafile.frame_height, frame);
            bafile.frames.push(baframe);
        } else {
            break;
        }
    }

    bafile.frame_count = bafile.frames.len() as u32;


    let mut writer = Cursor::new(Vec::new());
    bafile.write_le(&mut writer).unwrap();

    fs::write("./assets/badapple.ba", writer.into_inner()).unwrap();
    Ok(())
}

fn encode_frame(width: u8, height: u8, frame: Frame) -> converter::Frame{
    let mut sections = Vec::new();

    let mut current_color = 0u8;
    let mut current_count = 0u16;
    for y in 0..height {
        for x in 0..width {
            let rgb = frame
                .slice(ndarray::s![y as usize,x as usize, ..])
                .to_slice()
                .unwrap();

            let grayscale = rgb_to_bw(&rgb[0], &rgb[1], &rgb[2]);
            if grayscale == current_color {
                current_count += 1;
            }
            else {
                if current_count > 0{
                    sections.push(Section::new(current_color, current_count));
                }
                current_color = grayscale;
                current_count = 1;
            }
        }
    }

    if current_count > 0 {
        sections.push(Section::new(current_color, current_count))
    }

    converter::Frame::new(sections)
}

fn rgb_to_grayscale_avg(r: &u8, g: &u8, b: &u8) -> u8{
    ((r.clone() as u32 + g.clone() as u32 + b.clone() as u32) / 3) as u8
}

fn rgb_to_bw(r: &u8, g: &u8, b: &u8) -> u8{
    let grayscale = rgb_to_grayscale_avg(r, g, b);
    if grayscale>128{
        0xFF
    }
    else {
        0x00
    }
}