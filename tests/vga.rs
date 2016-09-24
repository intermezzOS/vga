extern crate vga;

use vga::Vga;

#[test]
fn create() {
    let mut mock_memory = vec![0u8; 25 * 80];

    unsafe { Vga::new(&mut mock_memory[0] as *mut u8) };
}
