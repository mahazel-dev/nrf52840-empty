#![no_std]
#![no_main]

use nrf52840_hal as _;
use defmt_rtt as _;


#[cortex_m_rt::entry]
fn main() -> !  {

    defmt::println!("Hello Rust people");
    exit();

}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked");
    exit()
}


pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
