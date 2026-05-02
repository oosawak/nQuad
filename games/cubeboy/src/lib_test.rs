static mut FRAMEBUFFER: [u32; 128 * 128] = [0xFF00FF; 128 * 128];

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        for y in 40..88 {
            for x in 40..88 {
                FRAMEBUFFER[y * 128 + x] = 0x00FF00;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn update(_l: i32, _r: i32, _j: i32) {}

#[no_mangle]
pub extern "C" fn render() {}

#[no_mangle]
pub extern "C" fn get_framebuffer() -> *const u32 {
    unsafe { FRAMEBUFFER.as_ptr() }
}
