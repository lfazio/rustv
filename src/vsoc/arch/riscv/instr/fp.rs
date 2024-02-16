use super::super::registers::RvRegisters;
use crate::vsoc::{
    arch::{riscv::{exception::RvException, registers::RvFpuRegisters}, types::Uint},
    bus::Bus,
};

pub fn load(
    x: &mut RvRegisters,
    f: &mut RvFpuRegisters,
    width: usize,
    rd: usize,
    rs1: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let mut value: Uint = if imm < 0 {
        let addr: u64 = u64::from(x.get(rs1)) - imm.unsigned_abs() as u64;

        match bus.fetch(width, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    } else {
        let addr: u64 = u64::from(x.get(rs1)) + imm.unsigned_abs() as u64;

        match bus.fetch(width, addr) {
            Ok(v) => Uint::new(v),
            Err(e) => return Err(RvException::from(e)),
        }
    };

    match width {
        4 => print!("flw\t{},{}({})\t# ", f.name(rd), imm, x.name(rs1)),
        8 => print!("fld\t{},{}({})\t# ", f.name(rd), imm, x.name(rs1)),
        16 => print!("flq\t{},{}({})\t# ", f.name(rd), imm, x.name(rs1)),
        _ => return Err(RvException::InstructionIllegal),
    }

    if width * 8 < f.len() {
        value.extend_with(f.len(), 0xff);
    }

    f.set(rd, &value);

    Ok(value)
}

pub fn store(
    x: &mut RvRegisters,
    f: &mut RvFpuRegisters,
    width: usize,
    rs1: usize,
    rs2: usize,
    imm: i32,
    bus: &mut Bus,
) -> Result<Uint, RvException> {
    let addr: u64 = if imm < 0 {
        u64::from(x.get(rs1)) - imm.unsigned_abs() as u64
    } else {
        u64::from(x.get(rs1)) + imm as u64
    };
    let value: Uint = f.get(rs2);

    match width {
        4 => println!("fsw\t{},{}({})\t# {}", f.name(rs2), imm, x.name(rs1), value),
        8 => println!("fsd\t{},{}({})\t# {}", f.name(rs2), imm, x.name(rs1), value),
        16 => println!("fsq\t{},{}({})\t# {}", f.name(rs2), imm, x.name(rs1), value),
        _ => return Err(RvException::InstructionIllegal),
    }

    match bus.store(width, addr, &Vec::<u8>::from(value.clone())) {
        None => Ok(value),
        Some(e) => Err(RvException::from(e)),
    }
}

