use crate::vsoc::arch::{riscv::registers::RvRegisters, types::Uint};

pub fn lui(reg: &mut RvRegisters, rd: usize, imm: i32) {
    print!("lui\t");

    reg.set(rd, &Uint::from(imm).sextend(reg.width(), 32));

    println!("{},{:#0x}", reg.name(rd), imm as u32 >> 12);
}
