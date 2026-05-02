const W: u32 = 128;
const H: u32 = 128;

static mut FRAMEBUFFER: [u32; 128 * 128] = [0; 128 * 128];

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        // Fill with test pattern
        for y in 0..128 {
            for x in 0..128 {
                let idx = (y * 128 + x) as usize;
                FRAMEBUFFER[idx] = if (x + y) % 16 < 8 { 0xFF00FF } else { 0x000000 };
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn update(_left: i32, _right: i32, _jump: i32) {}

#[no_mangle]
pub extern "C" fn render() {
    unsafe {
        // Draw a green rectangle
        for y in 40..88 {
            for x in 40..88 {
                let idx = (y * 128 + x) as usize;
                FRAMEBUFFER[idx] = 0x00FF00;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn get_framebuffer() -> *const u32 {
    unsafe { FRAMEBUFFER.as_ptr() }
}
