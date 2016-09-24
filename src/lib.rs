#![no_std]

use core::fmt;
use core::fmt::Write;

pub struct Vga {
    location: *mut u8,
    buffer: [u8; 25 * 80 * 2],
    position: usize,
}

impl Vga {
    pub unsafe fn new(location: *mut u8) -> Vga {
        Vga {
            location: location,
            buffer: [0; 25 * 80 * 2],
            position: 0,
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
        for b in s.bytes() {
            let i = self.position;

            self.buffer[i] = b;
            self.buffer[i + 1] = 0x02;

            self.position += 2;
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
        assert_eq!(vga.buffer[1],  0x02);
    }

    #[test]
    fn write_a_word() {
        let mut mock_memory = [0u8; 25 * 80];
        let mut vga = unsafe { Vga::new(mock_memory.as_mut_ptr()) };

        let word = "word";
        vga.write_str(word).unwrap();
      
        assert_eq!(vga.buffer[0], 'w' as u8);
        assert_eq!(vga.buffer[1], 0x02);
        assert_eq!(vga.buffer[2], 'o' as u8);
        assert_eq!(vga.buffer[3], 0x02);
        assert_eq!(vga.buffer[4], 'r' as u8);
        assert_eq!(vga.buffer[5], 0x02);
        assert_eq!(vga.buffer[6], 'd' as u8);
        assert_eq!(vga.buffer[7], 0x02);
    }

    #[test]
    fn write_multiple_words() {
        let mut mock_memory = [0u8; 25 * 80];
        let mut vga = unsafe { Vga::new(mock_memory.as_mut_ptr()) };

        vga.write_str("hello ").unwrap();
        vga.write_str("world").unwrap();
      
        assert_eq!(vga.buffer[0], 'h' as u8);
        assert_eq!(vga.buffer[1], 0x02);
        assert_eq!(vga.buffer[2], 'e' as u8);
        assert_eq!(vga.buffer[3], 0x02);
        assert_eq!(vga.buffer[4], 'l' as u8);
        assert_eq!(vga.buffer[5], 0x02);
        assert_eq!(vga.buffer[6], 'l' as u8);
        assert_eq!(vga.buffer[7], 0x02);
        assert_eq!(vga.buffer[8], 'o' as u8);
        assert_eq!(vga.buffer[9], 0x02);
        assert_eq!(vga.buffer[10], ' ' as u8);
        assert_eq!(vga.buffer[11], 0x02);
        assert_eq!(vga.buffer[12], 'w' as u8);
        assert_eq!(vga.buffer[13], 0x02);
        assert_eq!(vga.buffer[14], 'o' as u8);
        assert_eq!(vga.buffer[15], 0x02);
        assert_eq!(vga.buffer[16], 'r' as u8);
        assert_eq!(vga.buffer[17], 0x02);
        assert_eq!(vga.buffer[18], 'l' as u8);
        assert_eq!(vga.buffer[19], 0x02);
        assert_eq!(vga.buffer[20], 'd' as u8);
        assert_eq!(vga.buffer[21], 0x02);
    }
}
