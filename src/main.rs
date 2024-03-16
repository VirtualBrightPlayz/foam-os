#![no_main]
#![no_std]

extern crate alloc;
mod buffer;
use core::{fmt::Debug, mem};

use alloc::format;
use buffer::Buffer;
use log::{debug, error, info, warn};
use uefi::{entry, proto::{console::gop::GraphicsOutput, rng::Rng}, table::{boot::BootServices, Boot, SystemTable}, Handle, Result, Status};

fn get_random_usize(rng: &mut Rng) -> usize {
    let mut buf = [0; mem::size_of::<usize>()];
    rng.get_rng(None, &mut buf).expect("get_rng failed");
    usize::from_le_bytes(buf)
}

fn draw(bt: &BootServices) -> Result {
    info!("Getting Graphics Output...");
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>()?;
    info!("Found handle...");
    let mut gop = bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;
    info!("Found protocol!");

    info!("Getting Random Number Generator...");
    let rng_handle = bt.get_handle_for_protocol::<Rng>()?;
    info!("Found handle...");
    let mut rng = bt.open_protocol_exclusive::<Rng>(rng_handle)?;
    info!("Found protocol!");

    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width, height);

    loop {
        buffer.blit(&mut gop)?;

        let red = (get_random_usize(&mut rng) % 255) as u8;
        let green = (get_random_usize(&mut rng) % 255) as u8;
        let blue = (get_random_usize(&mut rng) % 255) as u8;

        for y in 0..height {
            for x in 0..width {
                let pixel = buffer.pixel(x, y).unwrap();
                pixel.red = red;
                pixel.green = green;
                pixel.blue = blue;
            }
        }
        bt.stall(0_500_000);
    }
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let bt = system_table.boot_services();
    info!("Stall!");
    bt.stall(2_000_000);
    let res = draw(bt);
    if res.is_err() {
        let err = res.unwrap_err();
        error!("Status code: {}", err.status());
        bt.stall(2_000_000);
        return Status::DEVICE_ERROR;
    }
    return Status::SUCCESS;
}
