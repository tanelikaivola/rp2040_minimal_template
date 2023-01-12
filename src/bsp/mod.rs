pub mod prelude;

pub use rp2040_hal as hal;

#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

hal::bsp_pins!(Gpio25 {
    name: led,
    aliases: { PushPullOutput: Led }
},);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
