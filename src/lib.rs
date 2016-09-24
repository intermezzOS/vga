pub struct Vga {
    location: *mut u8,
}

impl Vga {
    pub unsafe fn new(location: *mut u8) -> Vga {
        Vga { location: location }
    }
}
