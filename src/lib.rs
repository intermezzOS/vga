#![no_std]

use core::fmt;
use core::fmt::Write;

pub struct Vga {
    location: *mut u8,
    buffer: [u8; 25 * 80],
}

impl Vga {
    pub unsafe fn new(location: *mut u8) -> Vga {
        Vga {
            location: location,
            buffer: [0; 25 * 80],
        }
    }
}

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.buffer[0] = s.bytes().next().unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use Vga;
    use core::fmt::Write;

    #[test]
    fn write_a_letter() {
        let mut mock_memory = [0u8; 25 * 80];

        let mut vga = unsafe { Vga::new(&mut mock_memory[0] as *mut u8) };

        vga.write_str("a").unwrap();

        assert_eq!(vga.buffer[0], 'a' as u8);
    }
}
