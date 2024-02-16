use crate::vsoc::arch::types::Uint;

use super::super::registers::RvRegisters;

pub fn addi(x: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = x.get(rs1);

    if rs1 == 0 {
        if rd == 0 && imm == 0 {
            println!("nop");
            return;
        } else {
            print!("li\t{},{}\t# ", x.name(rd), imm);
        }
    } else if imm == 0 {
        print!("mv\t{},{}\t# ", x.name(rd), x.name(rs1));
    } else {
        print!("addi\t{},{},{}\t# ", x.name(rd), x.name(rs1), imm);
    }

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value.wrapping_add(imm);

            println!("{}", &Uint::from(result));
            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value.wrapping_add(imm as i64);

            println!("{}", &Uint::from(result));
            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value.wrapping_add(imm as i128);

            println!("{}", &Uint::from(result));
            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn addiw(x: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = x.get(rs1);

    if imm == 0 {
        print!("sext.w\t{},{}\t# ", x.name(rd), x.name(rs1));
    } else {
        print!("addiw\t{},{},{}\t# ", x.name(rd), x.name(rs1), imm);
    }

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value.wrapping_add(imm as i64) as i32 as i64;

            println!("{}", &Uint::from(result));
            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value.wrapping_add(imm as i128) as i32 as i128;

            println!("{}", &Uint::from(result));
            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn slti(x: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = x.get(rs1);

    println!("slti\t{},{},{}", x.name(rd), x.name(rs1), imm);

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = if rs1value < imm { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = if rs1value < imm as i64 { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = if rs1value < imm as i128 { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sltiu(x: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = x.get(rs1);

    println!("sltiu\t{},{},{}", x.name(rd), x.name(rs1), imm);

    match x.len() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = if rs1value < imm as u32 { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = if rs1value < imm as u64 { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = if rs1value < imm as u128 { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn andi(x: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = x.get(rs1);
    let i: Uint = Uint::from(imm).sextend(x.len(), 32);
    let result: Uint = rs1v & i;

    println!("andi\t{},{},{}", x.name(rd), x.name(rs1), imm);

    x.set(rd, &result);
}

pub fn ori(x: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = x.get(rs1);
    let i: Uint = Uint::from(imm).sextend(x.len(), 32);
    let result: Uint = rs1v | i;

    println!("ori\t{},{},{}", x.name(rd), x.name(rs1), imm);

    x.set(rd, &result);
}

pub fn xori(x: &mut RvRegisters, rd: usize, rs1: usize, imm: i32) {
    let rs1v: Uint = x.get(rs1);
    let i: Uint = Uint::from(imm).sextend(x.len(), 32);
    let result: Uint = rs1v ^ i;

    if imm == -1 {
        println!("not\t{},{}", x.name(rd), x.name(rs1));
    } else {
        println!("xori\t{},{},{}", x.name(rd), x.name(rs1), imm);
    }

    x.set(rd, &result);
}

pub fn slli(x: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = x.get(rs1);

    print!("slli\t{},{},{:#x}\t# ", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value << shamt;

            println!("{}", &Uint::from(result));
            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value << shamt;

            println!("{}", &Uint::from(result));
            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value << shamt;

            println!("{}", &Uint::from(result));
            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn slliw(x: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = x.get(rs1);

    println!("slliw\t{},{},{:#x}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: i64 = ((rs1value << shamt) & 0xffffffff) as i32 as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: i128 = ((rs1value << shamt) & 0xffffffff) as i32 as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn srli(x: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = x.get(rs1);

    println!("srli\t{},{},{:#x}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value.wrapping_shr(shamt as u32);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value.wrapping_shr(shamt as u32);

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value.wrapping_shr(shamt as u32);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn srliw(x: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = x.get(rs1);

    println!("srliw\t{},{},{:#x}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = (rs1value as u32).wrapping_shr(shamt as u32) as i32 as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = (rs1value as u32).wrapping_shr(shamt as u32) as i32 as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn srai(x: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = x.get(rs1);

    println!("srai\t{},{},{:#0x}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value >> shamt;

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value >> shamt;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value >> shamt;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sraiw(x: &mut RvRegisters, rd: usize, rs1: usize, shamt: usize) {
    let rs1v: Uint = x.get(rs1);

    println!("sraiw\t{},{},{}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = (rs1value as i32).wrapping_shr(shamt as u32) as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = (rs1value as i32).wrapping_shr(shamt as u32) as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}
