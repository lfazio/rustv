use crate::vsoc::arch::{riscv::registers::RvRegisters, types::Uint};

pub fn auipc(x: &mut RvRegisters, rd: usize, pc: &Uint, imm: i32) {
    print!("auipc\t{},{:08x}\t# ", x.name(rd), imm as u32 >> 12);

    match x.len() {
        32 => {
            println!("{}", &Uint::from(i32::from(pc.clone()).wrapping_add(imm)));

            x.set(rd, &Uint::from(i32::from(pc.clone()).wrapping_add(imm)));
        },
        64 => {
            println!("{}", &Uint::from(i64::from(pc.clone()).wrapping_add(imm as i64)));

            x.set(
            rd,
            &Uint::from(i64::from(pc.clone()).wrapping_add(imm as i64)));
        },
        128 =>  {
            println!("{}", &Uint::from(i128::from(pc.clone()).wrapping_add(imm as i128)));

            x.set(
            rd,
            &Uint::from(i128::from(pc.clone()).wrapping_add(imm as i128)));
        },
        _ => unreachable!(),
    };
}
