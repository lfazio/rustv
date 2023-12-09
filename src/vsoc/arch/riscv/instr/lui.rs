use crate::vsoc::arch::riscv::registers::RvRegisters;

pub fn lui(reg: &mut RvRegisters, rd: usize, imm: i32) {

    print!("lui\t");

    match reg.width() {
        32 => {
            reg.set(rd, &imm.to_le_bytes());
        },
        64 => {
            reg.set(rd, &(imm as i64).to_le_bytes());
        },
        128 => {
            reg.set(rd, &(imm as i128).to_le_bytes());
        },
        _ => unreachable!(),
    };

    println!("{},{:#0x}", reg.name(rd), imm as u32 >> 12);
}
