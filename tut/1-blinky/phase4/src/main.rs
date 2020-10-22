#![feature(asm)]
#![feature(global_asm)]

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

unsafe fn kmain() -> ! {
    // (addition sets register to pin16 ) wile OR op marks it as an output
    //define pin16 address
    let pin16: u32 = GPIO_FSEL1.read_volatile() + (16/10);

    // mark pin16 as output and store in FSEL1
    GPIO_FSEL1.write_volatile(pin16 | 1 << 18);

    // Infinite loop alternating on/off
    loop {
        GPIO_SET0.write_volatile(1 << 4);
        spin_sleep_ms(300);
        GPIO_CLR0.write_volatile(1 << 4);
    }
}
