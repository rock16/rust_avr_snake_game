#![no_std]
#![no_main]

mod display;
use panic_halt as _;

#[no_mangle]
pub extern "C" fn main() {
    loop {
        //do something
    }
}
