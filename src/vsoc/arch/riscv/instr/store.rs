use crate::vsoc::bus::Bus;
use super::super::registers::RvRegisters;

pub fn sb(reg: &mut RvRegisters, rs1: usize, rs2: usize, imm: i32, bus: &mut Bus) -> Vec<u8> {
    let addr: u64 = if imm < 0 {
        u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64
    } else {
        u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm as u64
    };
    let mut value: Vec<u8> = reg.get(rs2);

    value.truncate(1);
    value.shrink_to_fit();

    println!("sb\t{},{},{:x}", reg.name(rs2), reg.name(rs1), u8::from_le_bytes(value.clone().try_into().unwrap()));

    match bus.store(1, addr, &value) {
        None => value,
        _ => todo!(),
    }
}


pub fn sh(reg: &mut RvRegisters, rs1: usize, rs2: usize, imm: i32, bus: &mut Bus) -> Vec<u8> {
    let addr: u64  = if imm < 0 {
        u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64
    } else {
        u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm as u64
    };
    let mut value: Vec<u8> = reg.get(rs2);

    value.truncate(2);
    value.shrink_to_fit();

    println!("sh\t{},{},{:x}", reg.name(rs2), reg.name(rs1), u16::from_le_bytes(value.clone().try_into().unwrap()));

    match bus.store(2, addr, &value) {
        None => value,
        _ => todo!(),
    }
}

pub fn sw(reg: &mut RvRegisters, rs1: usize, rs2: usize, imm: i32, bus: &mut Bus) -> Vec<u8> {
    let addr: u64 = if imm < 0 {
        u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64
    } else {
        u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm as u64
    };
    let mut value: Vec<u8> = reg.get(rs2);

    value.truncate(4);
    value.shrink_to_fit();

    println!("sw\t{},{},{:x}", reg.name(rs2), reg.name(rs1), u32::from_le_bytes(value.clone().try_into().unwrap()));

    match bus.store(4, addr, &value) {
        None => value,
        _ => todo!(),
    }
}

pub fn sd(reg: &mut RvRegisters, rs1: usize, rs2: usize, imm: i32, bus: &mut Bus) -> Vec<u8> {
    let addr: u64 = if imm < 0 {
        u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64
    } else {
        u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm as u64
    };
    let mut value: Vec<u8> = reg.get(rs2);

    value.truncate(8);
    value.shrink_to_fit();

    println!("sd\t{},{},{:x}", reg.name(rs2), reg.name(rs1), u64::from_le_bytes(value.clone().try_into().unwrap()));

    match bus.store(8, addr, &value) {
        None => value,
        _ => todo!(),
    }
}
