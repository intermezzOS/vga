#![no_std]

use core::fmt;
use core::fmt::Write;

pub struct Vga {
    location: *mut u8,
}

impl Vga {
    pub unsafe fn new(location: *mut u8) -> Vga {
        Vga { location: location }
    }
}

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        Ok(())
    }
}
