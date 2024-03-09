#![no_std]
#![no_main]

use utils::{Reader, FrameManager};

mod utils;

const BADAPPLE: &[u8] = include_bytes!("../../assets/badapple.ba");

use gba::prelude::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
fn main() -> ! {
  DISPCNT.write(
    DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true),
  );

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
                let gba_color = Color::from_rgb(color as u16, color as u16, color as u16);
                let pixel_count = reader.read_u16();
                for _ in 0..pixel_count{
                    let (x, y) = frame_m.get_pos();
                    VIDEO3_VRAM.index(x as usize, y as usize).write(gba_color);
                }
            }

            frame_number+=1;
        }
        //vblank.wait_for_vblank();
    }
}



