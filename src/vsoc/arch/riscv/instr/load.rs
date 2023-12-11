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
    let mut value: Uint = if imm < 0 {
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

    let v = i8::from(value);
    println!("lb\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    value = match reg.width() {
        32 => Uint::from(v as i32),
        64 => Uint::from(v as i64),
        128 => Uint::from(v as i128),
        _ => unreachable!(),
    };
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
    let mut value: Uint = if imm < 0 {
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

    let v: i16 = i16::from(value.clone());
    println!("lh\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    value = match reg.width() {
        32 => Uint::from(v as i32),
        64 => Uint::from(v as i64),
        128 => Uint::from(v as i128),
        _ => unreachable!(),
    };
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
    let mut value: Uint = if imm < 0 {
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

    let v = i32::from(value.clone());
    println!("lw\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    value = match reg.width() {
        32 => Uint::from(v as i32),
        64 => Uint::from(v as i64),
        128 => Uint::from(v as i128),
        _ => unreachable!(),
    };
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
    let mut value: Uint = if imm < 0 {
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

    let v = i64::from(value.clone());
    println!("ld\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    value = match reg.width() {
        64 => Uint::from(v as i64),
        128 => Uint::from(v as i128),
        _ => unreachable!(),
    };
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
    let mut value: Uint = if imm < 0 {
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

    let v = u8::from(value.clone());
    println!("lbu\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    value = match reg.width() {
        32 => Uint::from(v as u32),
        64 => Uint::from(v as u64),
        128 => Uint::from(v as u128),
        _ => unreachable!(),
    };
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
    let mut value: Uint = if imm < 0 {
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

    let v = u16::from(value.clone());
    println!("lhu\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    value = match reg.width() {
        32 => Uint::from(v as u32),
        64 => Uint::from(v as u64),
        128 => Uint::from(v as u128),
        _ => unreachable!(),
    };
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
    let mut value: Uint = if imm < 0 {
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

    let v = u32::from(value.clone());
    println!("lwu\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    value = match reg.width() {
        32 => Uint::from(v as u32),
        64 => Uint::from(v as u64),
        128 => Uint::from(v as u128),
        _ => unreachable!(),
    };
    reg.set(rd, &value);

    Ok(value)
}
