#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(cfg_overflow_checks)]
#![feature(core_intrinsics)]
#![feature(stmt_expr_attributes)]
#![feature(f16)]

extern crate alloc;

mod test_subst;

use embedded_alloc::LlffHeap as Heap;

#[cdm_rt::entry]
fn main() -> ! {
    unsafe {
        #[global_allocator]
        static HEAP: Heap = Heap::empty();
        embedded_alloc::init!(HEAP, 2048);
    }

    test_subst::main();

    unsafe { core::arch::asm!("ldi r0, 0", "halt", options(noreturn)) }
}

#[cdm_rt::exception(Default)]
fn on_exception() -> ! {
    unsafe { core::arch::asm!("ldi r0, 0xDEAD", "halt", options(noreturn)) }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::arch::asm!("zero", options(noreturn)) }
}
