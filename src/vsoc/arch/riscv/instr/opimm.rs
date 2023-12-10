use crate::vsoc::arch::types::Uint;

use super::super::registers::RvRegisters;

pub fn addi(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = reg.get(rs1);

    if rs1 == 0 {
        if rd == 0 && imm == 0 {
            println!("nop");
        } else { 
            println!("li\t{},{}", reg.name(rd), imm);
        }
    } else if imm == 0 {
        println!("mv\t{},{}", reg.name(rd), reg.name(rs1));
    } else {
        println!("addi\t{},{},{}", reg.name(rd), reg.name(rs1), imm);
    }
    
    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value.wrapping_add(imm);

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value.wrapping_add(imm as i64);

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value .wrapping_add(imm as i128);

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn addiw(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = reg.get(rs1);

    if imm == 0 {
        println!("sext.w\t{},{}", reg.name(rd), reg.name(rs1));
    } else {
        println!("addiw\t{},{},{}", reg.name(rd), reg.name(rs1), imm);
    }
    
    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value.wrapping_add(imm as i64) as i32 as i64;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value.wrapping_add(imm as i128) as i32 as i128;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn slti(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = reg.get(rs1);

    println!("slti\t{},{},{}", reg.name(rd), reg.name(rs1), imm);

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = if rs1value < imm { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = if rs1value < imm as i64 { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = if rs1value < imm as i128 { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn sltiu(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = reg.get(rs1);

    println!("sltiu\t{},{},{}", reg.name(rd), reg.name(rs1), imm);

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = if rs1value < imm as u32 { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = if rs1value < imm as u64 { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = if rs1value < imm as u128 { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn andi(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = reg.get(rs1);

    println!("andi\t{},{},{}", reg.name(rd), reg.name(rs1), imm);

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value & imm;

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value & imm as i64;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value & imm as i128;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn ori(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = reg.get(rs1);

    println!("ori\t{},{},{}", reg.name(rd), reg.name(rs1), imm);

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value | imm;

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value | imm as i64;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value | imm as i128;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn xori(reg: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = reg.get(rs1);

    if imm == -1 {
        println!("not\t{},{}", reg.name(rd), reg.name(rs1));
    } else {
        println!("xori\t{},{},{}", reg.name(rd), reg.name(rs1), imm);
    }

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value ^ imm;

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value ^ imm as i64;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value ^ imm as i128;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn slli(reg: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = reg.get(rs1);

    println!("slli\t{},{},{:#x}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value << shamt;

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value << shamt;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value << shamt;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn slliw(reg: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = reg.get(rs1);

    println!("slliw\t{},{},{:#x}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: i64 = ((rs1value << shamt) & 0xffffffff) as i32 as i64;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: i128 = ((rs1value << shamt) & 0xffffffff) as i32 as i128;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn srli(reg: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = reg.get(rs1);

    println!("srli\t{},{},{:#x}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value.wrapping_shr(shamt as u32);

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value.wrapping_shr(shamt as u32);

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value.wrapping_shr(shamt as u32);

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn srliw(reg: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = reg.get(rs1);

    println!("srliw\t{},{},{:#x}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = (rs1value as u32).wrapping_shr(shamt as u32) as i32 as i64;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = (rs1value as u32).wrapping_shr(shamt as u32) as i32 as i128;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn srai(reg: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = reg.get(rs1);

    println!("srai\t{},{},{:#0x}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value >> shamt;

            reg.set(rd, &Uint::from(result));
        },
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value >> shamt;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value >> shamt;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}

pub fn sraiw(reg: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = reg.get(rs1);

    println!("sraiw\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = (rs1value as i32).wrapping_shr(shamt as u32) as i64;

            reg.set(rd, &Uint::from(result));
        },
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = (rs1value as i32).wrapping_shr(shamt as u32) as i128;

            reg.set(rd, &Uint::from(result));
        },
        _ => unreachable!(),
    }
}
