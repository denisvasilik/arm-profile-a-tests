use register::cpu::RegisterReadOnly;

use arm_isa_a32::instructions::{nop};
use arm_profile_a::register::{CPSR};

pub fn test_cpsr_n() -> Result<i32, ()> {
    let a: i32 = 1;
    let b: i32 = 2;
    let c: i32;

    unsafe {
        asm!("subs $2, $0, $1" : "=r"(c) : "r"(a), "r"(b) :: "volatile");
    };

    CPSR.is_set(CPSR::N);
    Err(())
}

pub fn test_cpsr_z() -> Result<i32, ()> {
    let a: i32 = 1;
    let b: i32 = 2;
    let c: i32;

    unsafe {
        asm!("subs $2, $0, $1" : "=r"(c) : "r"(a), "r"(b) :: "volatile");
    };

    CPSR.is_set(CPSR::N);
    Ok(0)
}

pub fn test_cpsr_c() -> Result<i32, ()> {
    let a: i32 = 1;
    let b: i32 = 2;
    let c: i32;

    unsafe {
        asm!("subs $2, $0, $1" : "=r"(c) : "r"(a), "r"(b) :: "volatile");
    };

    CPSR.is_set(CPSR::N);
    Err(())
}

pub fn test_cpsr_v() -> Result<i32, ()> {
    let a: i32 = 1;
    let b: i32 = 2;
    let c: i32;

    unsafe {
        asm!("subs $2, $0, $1" : "=r"(c) : "r"(a), "r"(b) :: "volatile");
    };

    CPSR.is_set(CPSR::N);
    Err(())
}

pub fn test_flush_cache() -> Result<i32, ()> {
    let c7format: u32 = 0;

    unsafe {
        asm!("mcr p15, 0, $0, c7, c7, 0" :: "r"(c7format) :: "volatile");
    }

    Ok(0)
}

pub fn test_flush_data_cache() -> Result<i32, ()> {
    let c7format: u32 = 0;

    unsafe {
        asm!("mcr p15, 0, $0, c7, c6, 0" :: "r"(c7format) :: "volatile");
    }

    Ok(0)
}

pub fn test_flush_instruction_cache() -> Result<i32, ()> {
    let c7format: u32 = 0;

    unsafe {
        asm!("mcr p15, 0, $0, c7, c5, 0" :: "r"(c7format) :: "volatile");
    }

    Ok(0)
}
