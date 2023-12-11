use crate::vsoc::arch::{riscv::registers::RvRegisters, types::Uint};

pub fn auipc(reg: &mut RvRegisters, rd: usize, pc: &Uint, imm: i32) {
    print!("auipc\t");

    match reg.width() {
        32 => reg.set(rd, &Uint::from(i32::from(pc.clone()).wrapping_add(imm))),
        64 => reg.set(
            rd,
            &Uint::from(i64::from(pc.clone()).wrapping_add(imm as i64)),
        ),
        128 => reg.set(
            rd,
            &Uint::from(i128::from(pc.clone()).wrapping_add(imm as i128)),
        ),
        _ => unreachable!(),
    };

    println!("{},{:#0x}", reg.name(rd), imm as u32 >> 12);
}
