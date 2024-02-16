use super::super::registers::RvRegisters;
use crate::vsoc::{
    arch::{riscv::exception::RvException, types::Uint},
    bus::Bus,
};

pub fn sb(
    reg: &mut RvRegisters,
    rs1: usize,
    rs2: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let addr: u64 = if imm < 0 {
        u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64
    } else {
        u64::from(reg.get(rs1)) + imm as u64
    };
    let mut value: Uint = reg.get(rs2);

    value.truncate(1);
    println!(
        "sb\t{},{}({})",
        reg.name(rs2),
        imm,
        reg.name(rs1)
    );

    match bus.store(1, addr, &Vec::<u8>::from(value.clone())) {
        None => Ok(value),
        Some(e) => Err(RvException::from(e)),
    }
}

pub fn sh(
    reg: &mut RvRegisters,
    rs1: usize,
    rs2: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let addr: u64 = if imm < 0 {
        u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64
    } else {
        u64::from(reg.get(rs1)) + imm as u64
    };
    let mut value: Uint = reg.get(rs2);

    value.truncate(2);
    println!(
        "sh\t{},{}({})\t# {}",
        reg.name(rs2),
        imm,
        reg.name(rs1),
        value
    );

    match bus.store(2, addr, &Vec::<u8>::from(value.clone())) {
        None => Ok(value),
        Some(e) => Err(RvException::from(e)),
    }
}

pub fn sw(
    reg: &mut RvRegisters,
    rs1: usize,
    rs2: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let addr: u64 = if imm < 0 {
        u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64
    } else {
        u64::from(reg.get(rs1)) + imm as u64
    };
    let mut value: Uint = reg.get(rs2);

    value.truncate(4);
    println!(
        "sw\t{},{}({})\t# {}",
        reg.name(rs2),
        imm,
        reg.name(rs1),
        value
    );

    match bus.store(4, addr, &Vec::<u8>::from(value.clone())) {
        None => Ok(value),
        Some(e) => Err(RvException::from(e)),
    }
}

pub fn sd(
    reg: &mut RvRegisters,
    rs1: usize,
    rs2: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let addr: u64 = if imm < 0 {
        u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64
    } else {
        u64::from(reg.get(rs1)) + imm as u64
    };
    let mut value: Uint = reg.get(rs2);

    value.truncate(8);
    println!(
        "sd\t{},{}({})\t# {}",
        reg.name(rs2),
        imm,
        reg.name(rs1),
        value
    );

    match bus.store(8, addr, &Vec::<u8>::from(value.clone())) {
        None => Ok(value),
        Some(e) => Err(RvException::from(e)),
    }
}
