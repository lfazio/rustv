use crate::vsoc::arch::{
    riscv::{
        csr::{Csr, self},
        exception::RvException,
    },
    types::Uint,
};

use super::super::registers::RvRegisters;

pub fn ecall(_reg: &mut RvRegisters) -> Option<RvException> {
    println!("ecall\ttodo\ttodo\ttodo");

    None
}

pub fn ebreak(_reg: &mut RvRegisters) -> Option<RvException> {
    println!("ebreak\ttodo\ttodo\ttodo");

    Some(RvException::Breakpoint)
}

pub fn xret(
    _reg: &mut RvRegisters,
    _rd: usize,
    _rs1: usize,
    _funct12: usize,
    _csr: &mut Csr,
) -> Option<RvException> {
    println!("xret\ttodo\ttodo\ttodo");

    None
}

pub fn csrrw(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    funct3: usize,
    funct12: usize,
    csr: &mut Csr,
) -> Option<RvException> {
    let value: Uint;
    let dest: Uint = match csr.get(funct12) {
        Some(d) => d,
        None => return Some(RvException::InstructionIllegal),
    };

    match funct3 {
        0x1 => {
            // csrrw
            value = match csr.get(funct12) {
                Some(d) => d,
                None => return Some(RvException::InstructionIllegal),
            };

            if rd == 0 {
                println!("csrw\t{},{}", csr.name(funct12), reg.name(rs1));
            } else {
                println!(
                    "csrrw\t{},{},{}",
                    csr.name(funct12),
                    reg.name(rd),
                    reg.name(rs1)
                );
            }
        }
        0x5 => {
            // csrrwi
            value = match reg.width() {
                32 => Uint::from((((rs1 << 26) as i32) >> 26) as u32),
                64 => Uint::from((((rs1 << 26) as i32) >> 26) as i64 as u64),
                128 => Uint::from((((rs1 << 26) as i32) >> 26) as i128 as u128),
                _ => return Some(RvException::InstructionIllegal),
            };

            if rd == 0 {
                println!("csrwi\t{},{}", csr.name(funct12), rs1);
            } else {
                println!("csrrwi\t{},{}", csr.name(funct12), rs1);
            }
        }
        _ => return Some(RvException::InstructionIllegal),
    }

    csr.set(funct12, &value);
    reg.set(rd, &dest);

    None
}

pub fn csrrs(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    funct3: usize,
    funct12: usize,
    csr: &mut Csr,
) -> Option<RvException> {
    let value: Uint;
    let dest: Uint = match csr.get(funct12) {
        Some(d) => d,
        None => return Some(RvException::InstructionIllegal),
    };

    if rs1 == 0 {
        println!("csrr\t{},{}", reg.name(rd), csr.name(funct12));
    } else {
        match funct3 {
            0x2 => {
                // csrrs
                value = match reg.width() {
                    32 => Uint::from(u32::from(dest.clone()) | u32::from(reg.get(rs1))),
                    64 => Uint::from(u64::from(dest.clone()) | u64::from(reg.get(rs1))),
                    128 => Uint::from(u128::from(dest.clone()) | u128::from(reg.get(rs1))),
                    _ => return Some(RvException::InstructionIllegal),
                };

                println!("csrs\t{},{:0x},{:0x}", reg.name(rd), funct12, rs1);
            },
            0x6 => {
                // csrrsi
                value = match reg.width() {
                    32 => Uint::from(u32::from(dest.clone()) | rs1 as u32),
                    64 => Uint::from(u64::from(dest.clone()) | rs1 as u64),
                    128 => Uint::from(u128::from(dest.clone()) | rs1 as u128),
                    _ => return Some(RvException::InstructionIllegal),
                };

                println!("csrrs\t{},{},{:0x}", reg.name(rd), csr.name(funct12), rs1);
            },
            _ => return Some(RvException::InstructionIllegal),
        }

        csr.set(funct12, &value);
    }

    reg.set(rd, &dest);

    None
}

pub fn csrrc(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    funct3: usize,
    funct12: usize,
    csr: &mut Csr,
) -> Option<RvException> {
    let value: Uint;
    let dest: Uint = match csr.get(funct12) {
        Some(d) => d,
        None => return Some(RvException::InstructionIllegal),
    };

    if rs1 == 0 {
        println!("csrr\t{},{}", reg.name(rd), csr.name(funct12));
    } else {
        match funct3 {
            0x3 => {
                // csrrc
                value = match reg.width() {
                    32 => Uint::from(u32::from(dest.clone()) & !u32::from(reg.get(rs1))),
                    64 => Uint::from(u64::from(dest.clone()) & !u64::from(reg.get(rs1))),
                    128 => Uint::from(u128::from(dest.clone()) & !u128::from(reg.get(rs1))),
                    _ => return Some(RvException::InstructionIllegal),
                };

                println!("csrc\t{},{},{:0x}", reg.name(rd), csr.name(funct12), rs1);
            },
            0x7 => {
                // csrrci
                value = match reg.width() {
                    32 => Uint::from(u32::from(dest.clone()) & !(rs1 as u32)),
                    64 => Uint::from(u64::from(dest.clone()) & !(rs1 as u64)),
                    128 => Uint::from(u128::from(dest.clone()) & !(rs1 as u128)),
                    _ => return Some(RvException::InstructionIllegal),
                };

                println!("csrrc\t{},{},{:0x}", reg.name(rd), csr.name(funct12), rs1);
            },
            _ => return Some(RvException::InstructionIllegal),
        }

        csr.set(funct12, &value);
    }

    reg.set(rd, &dest);

    None
}
