#![no_std]
#![no_main]

// The crate has to be imported at some point.
// I suggest having the user tag the main function with #[entry] to work around this.
// Such a solution would both insure the crate has been imported and gets linked
// into the application, and will automatically format the main function so it can be
// found by our bootloader.
extern crate gba;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
  unsafe {
    (0x400_0000 as *mut u16).write_volatile(0x0403);
    (0x600_0000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x600_0000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x600_0000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
    loop {}
  }
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}
