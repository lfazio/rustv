use crate::vsoc::{bus::Bus, arch::riscv::exception::RvException};
use super::super::registers::RvRegisters;

pub fn lb(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32, bus: &mut Bus) -> Result<Vec<u8>, RvException> {
    let mut value: Vec<u8> = if imm < 0 {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64;

        match bus.fetch(1, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm.unsigned_abs() as u64;

        match bus.fetch(1, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    };

    let v = i8::from_le_bytes(value.clone().try_into().unwrap());
    println!("lb\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    match reg.width() {
        32 => {
            value = v.to_le_bytes().to_vec();
        },
        64 => {
            let val: i64 = v as i64;
            value = val.to_le_bytes().to_vec();
        },
        128 => {
            let val: i128 = v as i128;
            value = val.to_le_bytes().to_vec();
        },
        _ => unreachable!(),
    }
    reg.set(rd, &value);

    Ok(value)
}

pub fn lh(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32, bus: &mut Bus) -> Result<Vec<u8>, RvException> {
    let mut value: Vec<u8> = if imm < 0 {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64;

        match bus.fetch(2, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm.unsigned_abs() as u64;

        match bus.fetch(2, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    };

    let v = i16::from_le_bytes(value.clone().try_into().unwrap());
    println!("lh\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    match reg.width() {
        32 => {
            value = v.to_le_bytes().to_vec();
        },
        64 => {
            let val: i64 = v as i64;
            value = val.to_le_bytes().to_vec();
        },
        128 => {
            let val: i128 = v as i128;
            value = val.to_le_bytes().to_vec();
        },
        _ => unreachable!(),
    }
    reg.set(rd, &value);

    Ok(value)
}

pub fn lw(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32, bus: &mut Bus) -> Result<Vec<u8>, RvException> {
    let mut value: Vec<u8> = if imm < 0 {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64;

        match bus.fetch(4, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm.unsigned_abs() as u64;

        match bus.fetch(4, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    };

    let v = i32::from_le_bytes(value.clone().try_into().unwrap());
    println!("lw\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    match reg.width() {
        32 => {
            value = v.to_le_bytes().to_vec();
        },
        64 => {
            let val: i64 = v as i64;
            value = val.to_le_bytes().to_vec();
            },
        128 => {
            let val: i128 = v as i128;
            value = val.to_le_bytes().to_vec();
        },
        _ => unreachable!(),
    }
    reg.set(rd, &value);

    Ok(value)
}

pub fn ld(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32, bus: &mut Bus) -> Result<Vec<u8>, RvException> {
    let mut value: Vec<u8> = if imm < 0 {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64;

        match bus.fetch(8, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm.unsigned_abs() as u64;

        match bus.fetch(8, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    };

    let v = i64::from_le_bytes(value.clone().try_into().unwrap());
    println!("ld\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    match reg.width() {
        64 => {
            value = v.to_le_bytes().to_vec();
        },
        128 => {
            let val: i128 = v as i128;
            value = val.to_le_bytes().to_vec();
        },
        _ => unreachable!(),
    }
    reg.set(rd, &value);

    Ok(value)
}

pub fn lbu(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32, bus: &mut Bus) -> Result<Vec<u8>, RvException> {
    let mut value: Vec<u8> = if imm < 0 {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64;

        match bus.fetch(1, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm.unsigned_abs() as u64;

        match bus.fetch(1, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    };

    let v = u8::from_le_bytes(value.clone().try_into().unwrap());
    println!("lbu\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    match reg.width() {
        32 => {
            value = v.to_le_bytes().to_vec();
        },
        64 => {
            let val: u64 = v as u64;
            value = val.to_le_bytes().to_vec();
        },
        128 => {
            let val: u128 = v as u128;
            value = val.to_le_bytes().to_vec();
        },
        _ => unreachable!(),
    }
    reg.set(rd, &value);

    Ok(value)
}

pub fn lhu(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32, bus: &mut Bus) -> Result<Vec<u8>, RvException> {
    let mut value: Vec<u8> = if imm < 0 {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64;

        match bus.fetch(2, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm.unsigned_abs() as u64;

        match bus.fetch(2, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    };

    let v = u16::from_le_bytes(value.clone().try_into().unwrap());
    println!("lhu\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    match reg.width() {
        32 => {
            value = v.to_le_bytes().to_vec();
        },
        64 => {
            let val: u64 = v as u64;
            value = val.to_le_bytes().to_vec();
        },
        128 => {
            let val: u128 = v as u128;
            value = val.to_le_bytes().to_vec();
        },
        _ => unreachable!(),
    }
    reg.set(rd, &value);

    Ok(value)
}

pub fn lwu(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32, bus: &mut Bus) -> Result<Vec<u8>, RvException> {
    let mut value: Vec<u8> = if imm < 0 {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) - imm.unsigned_abs() as u64;

        match bus.fetch(4, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from_le_bytes(reg.get(rs1).try_into().unwrap()) + imm.unsigned_abs() as u64;

        match bus.fetch(4, addr) {
            Ok(v) => v,
            Err(e) => return Err(RvException::from(e)),
        }
    };

    let v = u32::from_le_bytes(value.clone().try_into().unwrap());
    println!("lwu\t{},{},{:x}", reg.name(rd), reg.name(rs1), v);

    match reg.width() {
        32 => {
            value = v.to_le_bytes().to_vec();
        },
        64 => {
            let val: u64 = v as u64;
            value = val.to_le_bytes().to_vec();
        },
        128 => {
            let val: u128 = v as u128;
            value = val.to_le_bytes().to_vec();
        },
        _ => unreachable!(),
    }
    reg.set(rd, &value);

    Ok(value)
}
