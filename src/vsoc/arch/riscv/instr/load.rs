use super::super::registers::RvRegisters;
use crate::vsoc::{
    arch::{riscv::exception::RvException, types::Uint},
    bus::Bus,
};

pub fn lb(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let addr: u64;
    let value: Uint = if imm < 0 {
        addr = u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64;

        match bus.fetch(1, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        addr = u64::from(reg.get(rs1)) + imm.unsigned_abs() as u64;

        match bus.fetch(1, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    };

    println!("lb\t{},{}({})\t# @{:0x}={}", reg.name(rd), imm, reg.name(rs1), addr, value);

    reg.set(rd, &value);

    Ok(value)
}

pub fn lh(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let value: Uint = if imm < 0 {
        let addr: u64 = u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64;

        match bus.fetch(2, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from(reg.get(rs1)) + imm.unsigned_abs() as u64;

        match bus.fetch(2, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    };

    println!("lh\t{},{}({})\t# {}", reg.name(rd), imm, reg.name(rs1), value);

    reg.set(rd, &value);

    Ok(value)
}

pub fn lw(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let addr: u64;
    let value: Uint = if imm < 0 {
        addr = u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64;

        match bus.fetch(4, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        addr = u64::from(reg.get(rs1)) + imm.unsigned_abs() as u64;

        match bus.fetch(4, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    };

    println!("lw\t{},{}({})\t# @{:0x}={}", reg.name(rd), imm, reg.name(rs1), addr, value);

    reg.set(rd, &value);

    Ok(value)
}

pub fn ld(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let value: Uint = if imm < 0 {
        let addr: u64 = u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64;

        match bus.fetch(8, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from(reg.get(rs1)) + imm.unsigned_abs() as u64;

        match bus.fetch(8, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    };

    println!("ld\t{},{}({})\t# {}", reg.name(rd), imm, reg.name(rs1), value);

    reg.set(rd, &value);

    Ok(value)
}

pub fn lbu(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let value: Uint = if imm < 0 {
        let addr: u64 = u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64;

        match bus.fetch(1, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from(reg.get(rs1)) + imm.unsigned_abs() as u64;

        match bus.fetch(1, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    };

    println!("lbu\t{},{}({})\t# {}", reg.name(rd), imm, reg.name(rs1), value);

    reg.set(rd, &value);

    Ok(value)
}

pub fn lhu(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let value: Uint = if imm < 0 {
        let addr: u64 = u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64;

        match bus.fetch(2, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from(reg.get(rs1)) + imm.unsigned_abs() as u64;

        match bus.fetch(2, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    };

    println!("lhu\t{},{}({})\t# {}", reg.name(rd), imm, reg.name(rs1), value);

    reg.set(rd, &value);

    Ok(value)
}

pub fn lwu(
    reg: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let value: Uint = if imm < 0 {
        let addr: u64 = u64::from(reg.get(rs1)) - imm.unsigned_abs() as u64;

        match bus.fetch(4, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from(reg.get(rs1)) + imm.unsigned_abs() as u64;

        match bus.fetch(4, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    };

    println!("lwu\t{},{}({})\t# {}", reg.name(rd), imm, reg.name(rs1), value);

    reg.set(rd, &value);

    Ok(value)
}
