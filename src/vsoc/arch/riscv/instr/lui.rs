use crate::vsoc::arch::{riscv::registers::RvRegisters, types::Uint};

pub fn lui(x: &mut RvRegisters, rd: usize, imm: i32) {
    print!("lui\t");

    x.set(rd, &Uint::from(imm).sextend(x.len(), 32));

    println!("{},{:#0x}", x.name(rd), imm as u32 >> 12);
}
