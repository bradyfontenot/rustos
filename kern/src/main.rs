#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

// use console::kprintln;

// FIXME: You need to add dependencies here to
// test your drivers (Phase 2). Add them as needed.
use pi::uart;
use pi::gpio;
use pi::timer;

fn kmain() -> ! {
    // FIXME: Start the shell.
    // unimplemented!()
    let uart = MiniUart::new();

    loop{
        uart.write(uart.read());
    }
}
