#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    write(1, b"Hello, world!\n");
    exit(0);
}

fn write(fd: i32, buf: &[u8]) {
    let ptr = buf.as_ptr();
    let len = buf.len();

    unsafe {
        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") fd,
            in("rsi") ptr,
            in("rdx") len,
            lateout("rax") _,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags),
        );
    }
}

fn exit(status: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") status,
            options(noreturn),
        );
    }
}
