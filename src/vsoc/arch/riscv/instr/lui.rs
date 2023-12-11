use crate::vsoc::arch::{riscv::registers::RvRegisters, types::Uint};

pub fn lui(reg: &mut RvRegisters, rd: usize, imm: i32) {
    print!("lui\t");

    match reg.width() {
        32 => {
            reg.set(rd, &Uint::from(imm));
        }
        64 => {
            reg.set(rd, &Uint::from(imm as i64));
        }
        128 => {
            reg.set(rd, &Uint::from(imm as i128));
        }
        _ => unreachable!(),
    };

    println!("{},{:#0x}", reg.name(rd), imm as u32 >> 12);
}
