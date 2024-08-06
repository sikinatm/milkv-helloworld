#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;


const UART0_THR: *mut u8 = 0x04140000 as *mut u8;
const UART0_LSR: *mut u8 = 0x04140014 as *mut u8;

global_asm!(r#"
.section .text
.global _start
_start:
    /* BL33 information */
    j real_start
    .balign 4
    .word 0x33334c42  /* b'BL33' */
    .word 0xdeadbeea  /* CKSUM */
    .word 0xdeadbeeb  /* SIZE */
    .quad 0x80200000  /* RUNADDR */
    .word 0xdeadbeec
    .balign 4
    j real_start
    .balign 4
    /* Information end */

real_start:
    /* Real start of the program */
    la sp, _stack_start
    call rust_main
"#);

extern "C" {
    static _stack_start: u8;
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let message = b"Hello, Milk-V with Rust!\n";
    for &c in message {
        unsafe {
            while UART0_LSR.read_volatile() & 0x20 == 0 {}
            UART0_THR.write_volatile(c);
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}