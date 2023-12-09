use crate::vsoc::arch::riscv::{csr::{Csr, self}, exception::RvException};

use super::super::registers::RvRegisters;

pub fn ecall(reg: &mut RvRegisters, csr: &mut Csr) -> Option<RvException> {
    let status: Vec<u8>;

    println!("ecall");

    status = match csr.get(csr::MSTATUS) {
        Some(s) => s,
        None => return Some(RvException::InstructionIllegal),
    };

    None
}

pub fn ebreak(reg: &mut RvRegisters, csr: &mut Csr) -> Option<RvException> {
    println!("ebreak\ttodo\ttodo\ttodo");
    Some(RvException::Breakpoint)
}

pub fn xret(reg: &mut RvRegisters, rd: usize, rs1: usize, funct12: u16, csr: &mut Csr) -> Option<RvException> {
    todo!("xret");
}

pub fn csrrw(reg: &mut RvRegisters, rd: usize, rs1: usize, funct3: usize, funct12: u16, csr: &mut Csr) -> Option<RvException> {
    let value: Vec<u8>;
    let dest: Vec<u8> = match csr.get(funct12 as usize) {
        Some(d) => d,
        None => return Some(RvException::InstructionIllegal),
    };

    match funct3 {
        0x1 => { // csrrw
            value = match csr.get(funct12 as usize) {
                Some(d) => d,
                None => return Some(RvException::InstructionIllegal),
            };

            if rd == 0 {
                println!("csrw\t{},{}", csr.name(funct12 as usize), reg.name(rs1));
            } else {
                println!("csrrw\t{},{},{}", csr.name(funct12 as usize), reg.name(rd), reg.name(rs1));
            }
        },
        0x5 => { // csrrwi
            match reg.width() {
                32 => value = ((((rs1 << 26) as i32) >> 26) as u32).to_le_bytes().to_vec(),
                64 => value = ((((rs1 << 26) as i32) >> 26) as i64 as u64).to_le_bytes().to_vec(),
                128 => value = ((((rs1 <<26) as i32) >> 26) as i128 as u128).to_le_bytes().to_vec(),
                _ => return Some(RvException::InstructionIllegal),
            }

            if rd == 0 {
                println!("csrwi\t{},{}", csr.name(funct12 as usize), rs1);
            } else {
                println!("csrrwi\t{},{}", csr.name(funct12 as usize), rs1);
            }
        },
        _ => return Some(RvException::InstructionIllegal),
    }

    csr.set(funct12 as usize, &value);
    reg.set(rd, &dest);

    None
}

pub fn csrrs(reg: &mut RvRegisters, rd: usize, rs1: usize, funct3: usize, funct12: u16, csr: &mut Csr) -> Option<RvException> {
    let value: Vec<u8>;
    let dest: Vec<u8> = match csr.get(funct12 as usize) {
        Some(d) => d,
        None => return Some(RvException::InstructionIllegal),
    };

    if rs1 == 0 {
        println!("csrr\t{},{}", reg.name(rd), csr.name(funct12 as usize));
    } else {
        match funct3 {
            0x2 => { // csrrs
                match reg.width() {
                    32 => {
                        let rs1v: u32 = u32::from_le_bytes(reg.get(rs1).try_into().unwrap());
                    
                        value = (u32::from_le_bytes(dest.clone().try_into().unwrap()) | rs1v).to_le_bytes().to_vec();
                    },
                    64 => {
                        let rs1v: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap());
                        
                        value = (u64::from_le_bytes(dest.clone().try_into().unwrap()) | rs1v).to_le_bytes().to_vec();
                    },
                    128 => {
                        let rs1v: u128 = u128::from_le_bytes(reg.get(rs1).try_into().unwrap());
                        
                        value = (u128::from_le_bytes(dest.clone().try_into().unwrap()) | rs1v).to_le_bytes().to_vec();
                    },
                    _ => return Some(RvException::InstructionIllegal),
                }

                println!("csrs\t{},{:0x},{:0x}", reg.name(rd), funct12, rs1);
            },
            0x6 => { // csrrsi
                match reg.width() {
                    32 => value = (u32::from_le_bytes(dest.clone().try_into().unwrap()) | (rs1 as u32)).to_le_bytes().to_vec(),
                    64 => value = (u64::from_le_bytes(dest.clone().try_into().unwrap()) | (rs1 as u64)).to_le_bytes().to_vec(),
                    128 => value = (u128::from_le_bytes(dest.clone().try_into().unwrap()) | (rs1 as u128)).to_le_bytes().to_vec(),
                    _ => return Some(RvException::InstructionIllegal),
                }

                println!("csrrs\t{},{},{:0x}", reg.name(rd), csr.name(funct12 as usize), rs1);
            },
            _ => return Some(RvException::InstructionIllegal),
        }

        csr.set(funct12 as usize, &value);
    }

    reg.set(rd, &dest);

    None
}

pub fn csrrc(reg: &mut RvRegisters, rd: usize, rs1: usize, funct3: usize, funct12: u16, csr: &mut Csr) -> Option<RvException> {
    let value: Vec<u8>;
    let dest: Vec<u8> = match csr.get(funct12 as usize) {
        Some(d) => d,
        None => return Some(RvException::InstructionIllegal),
    };

    if rs1 == 0 {
        println!("csrr\t{},{}", reg.name(rd), csr.name(funct12 as usize));
    } else {
        match funct3 {
            0x3 => { // csrrc
                match reg.width() {
                    32 => {
                        let rs1v: u32 = u32::from_le_bytes(reg.get(rs1).try_into().unwrap());
                    
                        value = (u32::from_le_bytes(dest.clone().try_into().unwrap()) & !rs1v).to_le_bytes().to_vec();
                    },
                    64 => {
                        let rs1v: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap());
                        
                        value = (u64::from_le_bytes(dest.clone().try_into().unwrap()) & !rs1v).to_le_bytes().to_vec();
                    },
                    128 => {
                        let rs1v: u128 = u128::from_le_bytes(reg.get(rs1).try_into().unwrap());
                        
                        value = (u128::from_le_bytes(dest.clone().try_into().unwrap()) & !rs1v).to_le_bytes().to_vec();
                    },
                    _ => return Some(RvException::InstructionIllegal),
                }

                println!("csrc\t{},{},{:0x}", reg.name(rd), csr.name(funct12 as usize), rs1);
            },
            0x7 => { // csrrci
                match reg.width() {
                    32 => value = (u32::from_le_bytes(dest.clone().try_into().unwrap()) & !(rs1 as u32)).to_le_bytes().to_vec(),
                    64 => value = (u64::from_le_bytes(dest.clone().try_into().unwrap()) & !(rs1 as u64)).to_le_bytes().to_vec(),
                    128 => value = (u128::from_le_bytes(dest.clone().try_into().unwrap()) & !(rs1 as u128)).to_le_bytes().to_vec(),
                    _ => return Some(RvException::InstructionIllegal),
                }

                println!("csrrc\t{},{},{:0x}", reg.name(rd), csr.name(funct12 as usize), rs1);
            },
            _ => return Some(RvException::InstructionIllegal),
        }

        csr.set(funct12 as usize, &value);
    }

    reg.set(rd, &dest);

    None
}
