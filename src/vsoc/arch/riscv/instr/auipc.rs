use crate::vsoc::arch::{riscv::registers::RvRegisters, types::Uint};

pub fn auipc(reg: &mut RvRegisters, rd: usize, pc: &Uint, imm: i32) {
    print!("auipc\t{},{:08x}\t# ", reg.name(rd), imm as u32 >> 12);

    match reg.width() {
        32 => {
            println!("{}", &Uint::from(i32::from(pc.clone()).wrapping_add(imm)));

            reg.set(rd, &Uint::from(i32::from(pc.clone()).wrapping_add(imm)));
        },
        64 => {
            println!("{}", &Uint::from(i64::from(pc.clone()).wrapping_add(imm as i64)));

            reg.set(
            rd,
            &Uint::from(i64::from(pc.clone()).wrapping_add(imm as i64)));
        },
        128 =>  {
            println!("{}", &Uint::from(i128::from(pc.clone()).wrapping_add(imm as i128)));

            reg.set(
            rd,
            &Uint::from(i128::from(pc.clone()).wrapping_add(imm as i128)));
        },
        _ => unreachable!(),
    };
}
