#![no_std]
#![no_main]

use utils::{Reader, FrameManager};

mod utils;

const BADAPPLE: &[u8] = include_bytes!("../../assets/badapple.ba");

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    run_badapple(gba)
}

fn run_badapple(mut gba: agb::Gba) -> ! {
    let mut video = gba.display.video.bitmap3();
    let vblank = agb::interrupt::VBlank::get();

    let mut reader = Reader::new(&BADAPPLE);
    let mut frame_number = 0;

    let height = reader.read_u8();
    let width = reader.read_u8();
    let framerate = reader.read_u8();
    let frame_count = reader.read_u32();
    
    loop {
        if frame_number < frame_count{
            let sections_in_frame = reader.read_u16();
            let mut frame_m = FrameManager::new(width, height);

            for i in 0..sections_in_frame {
                let color = reader.read_u8();
                let pixel_count = reader.read_u16();
                for _ in 0..pixel_count{
                    let (x, y) = frame_m.get_pos();
                    video.draw_point(x as i32, y as i32, if color == 0xFF { u16::MAX} else {0});
                }
            }

            frame_number+=1;
        }
        vblank.wait_for_vblank();
    }
}


