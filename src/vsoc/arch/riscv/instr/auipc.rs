use crate::vsoc::arch::riscv::registers::RvRegisters;

pub fn auipc(reg: &mut RvRegisters, rd: usize, pc: &[u8], imm: i32) {

    print!("auipc\t");

    match reg.width() {
        32 => {
            let value: u32 = u32::from_le_bytes((*pc).try_into().unwrap());

            reg.set(rd, &(value as i32 + imm).to_le_bytes());
        },
        64 => {
            let value: u64 = u64::from_le_bytes((*pc).try_into().unwrap());

            reg.set(rd, &(value as i64 + imm as i64).to_le_bytes());
        },
        128 => {
            let value: u128 = u128::from_le_bytes((*pc).try_into().unwrap());

            reg.set(rd, &(value as i128 + imm as i128).to_le_bytes());
        },
        _ => unreachable!(),
    };

    println!("{},{:#0x}", reg.name(rd), imm as u32 >> 12);
}