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

    pub fn flush(&self) {
        unsafe {
            let location = self.location;
            let length = self.buffer.len();
            let buffer = self.buffer.as_ptr();

            core::ptr::copy_nonoverlapping(buffer, location, length);
        }
    }
}

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for (i, b)  in s.bytes().enumerate() {
            self.buffer[i] = b;
        }
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

        let mut vga = unsafe { Vga::new(mock_memory.as_mut_ptr()) };

        vga.write_str("a").unwrap();

        assert_eq!(vga.buffer[0], 'a' as u8);
    }

    #[test]
    fn write_a_word() {
        let mut mock_memory = [0u8; 25 * 80];
        let mut vga = unsafe { Vga::new(mock_memory.as_mut_ptr()) };

        let word = "word";
        vga.write_str(word).unwrap();
      
        assert_eq!(vga.buffer[0], 'w' as u8);
        assert_eq!(vga.buffer[1], 'o' as u8);
        assert_eq!(vga.buffer[2], 'r' as u8);
        assert_eq!(vga.buffer[3], 'd' as u8);
    }
}
