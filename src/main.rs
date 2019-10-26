#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(global_asm)]
#![feature(asm)]
#![no_std]
#![no_main]

extern crate compiler_builtins;
extern crate panic_abort;

mod tests;
mod test;

use arm_semihosting::{debug};

use test::{
    TestApp,
    TestRunner,
    TestCase,
};

global_asm!(include_str!("start.S"));

#[no_mangle]
fn _entry() -> ! {
    let mut app = TestApp::new();
    let mut runner = TestRunner::new();

    runner.tests[0] = Some(TestCase::new("test_cpsr_n", tests::test_cpsr_n));
    runner.tests[1] = Some(TestCase::new("test_cpsr_z", tests::test_cpsr_z));
    runner.tests[2] = Some(TestCase::new("test_cpsr_c", tests::test_cpsr_c));
    runner.tests[3] = Some(TestCase::new("test_cpsr_v", tests::test_cpsr_v));
    runner.tests[4] = Some(TestCase::new("test_flush_cache", tests::test_flush_cache));
    runner.tests[5] = Some(TestCase::new("test_flush_data_cache", tests::test_flush_cache));
    runner.tests[6] = Some(TestCase::new("test_flush_instruction_cache", tests::test_flush_cache));

    app.runner = Some(runner);

    app.run();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() {
}

#[no_mangle]
pub extern fn rust_begin_unwind() {
}

#[no_mangle]
pub extern fn _Unwind_Resume() {
}
