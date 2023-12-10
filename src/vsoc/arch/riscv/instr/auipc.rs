use crate::vsoc::arch::{riscv::registers::RvRegisters, types::Uint};

pub fn auipc(reg: &mut RvRegisters, rd: usize, pc: &Uint, imm: i32) {

    print!("auipc\t");

    match reg.width() {
        32 => {
            let value: u32 = u32::from(pc.clone());

            reg.set(rd, &Uint::from(value as i32 + imm));
        },
        64 => {
            let value: u64 = u64::from(pc.clone());

            reg.set(rd, &Uint::from(value as i64 + imm as i64));
        },
        128 => {
            let value: u128 = u128::from(pc.clone());

            reg.set(rd, &Uint::from(value as i128 + imm as i128));
        },
        _ => unreachable!(),
    };

    println!("{},{:#0x}", reg.name(rd), imm as u32 >> 12);
}