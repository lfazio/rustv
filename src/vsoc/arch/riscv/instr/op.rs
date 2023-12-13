use crate::vsoc::arch::types::Uint;

use super::super::registers::RvRegisters;

pub fn add(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("add\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);
            let result: i32 = rs1value.wrapping_add(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = rs1value.wrapping_add(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = rs1value.wrapping_add(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn addw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("addw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = (rs1value as i32).wrapping_add(rs2value as i32) as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = (rs1value as i32).wrapping_add(rs2value as i32) as i128;

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sub(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("sub\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);
            let result: i32 = rs1value.wrapping_sub(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = rs1value.wrapping_sub(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = rs1value.wrapping_sub(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn subw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("subw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = (rs1value as i32).wrapping_sub(rs2value as i32) as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = (rs1value as i32).wrapping_sub(rs2value as i32) as i128;

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn slt(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("slt\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);
            let result: i32 = if rs1value < rs2value { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = if rs1value < rs2value { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = if rs1value < rs2value { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sltu(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("sltu\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let result: u32 = if rs1v < rs2v { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let result: u64 = if rs1v < rs2v { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let result: u128 = if rs1v < rs2v { 1 } else { 0 };

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn and(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);
    let result: Uint = rs1v & rs2v;

    println!("and\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    reg.set(rd, &result);
}

pub fn or(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);
    let result: Uint = rs1v | rs2v;

    println!("or\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    reg.set(rd, &result);
}

pub fn xor(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);
    let result: Uint = rs1v ^ rs2v;

    println!("xor\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    reg.set(rd, &result);
}

pub fn sll(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let shamt: u32 = u32::from(reg.get(rs2)) & 0x1f;

    println!("sll\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value.wrapping_shl(shamt);

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value.wrapping_shl(shamt);

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value.wrapping_shl(shamt);

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sllw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let shamt: u32 = u32::from(reg.get(rs2)) & 0x1f;

    println!("sllw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: i64 = rs1value.wrapping_shl(shamt) as i32 as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: i128 = rs1value.wrapping_shl(shamt) as i32 as i128;

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn srl(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let shamt: u32 = u32::from(reg.get(rs2)) & 0x1f;

    println!("srl\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value.wrapping_shr(shamt);

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value >> shamt;
            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value >> shamt;

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn srlw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let shamt: u32 = u32::from(reg.get(rs2)) & 0x1f;

    println!("srlw\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = (rs1value as u32).wrapping_shr(shamt) as i32 as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = (rs1value as u32).wrapping_shr(shamt) as i32 as i128;

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sra(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let shamt: u32 = u32::from(reg.get(rs2)) & 0x1f;

    println!("sra\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value.wrapping_shr(shamt);

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value.wrapping_shr(shamt);

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value.wrapping_shr(shamt);

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sraw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let shamt: u32 = u32::from(reg.get(rs2)) & 0x1f;

    println!("sraw\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = (rs1value as i32 >> shamt) as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = (rs1value as i32 >> shamt) as i128;

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn mul(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("mul\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);
            let result: i32 = rs1value.wrapping_mul(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = rs1value.wrapping_mul(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = rs1value.wrapping_mul(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn mulh(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("mulh\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i64 = i32::from(rs1v) as i64;
            let rs2value: i64 = i32::from(rs2v) as i64;
            let result: i32 = (rs1value.wrapping_mul(rs2value) >> reg.width()) as i32;

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i128 = i64::from(rs1v) as i128;
            let rs2value: i128 = i64::from(rs2v) as i128;
            let result: i64 = (rs1value.wrapping_mul(rs2value) >> reg.width()) as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            todo!();
        }
        _ => unreachable!(),
    }
}

pub fn mulhu(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!(
        "mulhu\t{},{},{}",
        reg.name(rd),
        reg.name(rs1),
        reg.name(rs2)
    );

    match reg.width() {
        32 => {
            let rs1value: u64 = u32::from(rs1v) as u64;
            let rs2value: u64 = u32::from(rs2v) as u64;
            let result: u32 = (rs1value.wrapping_mul(rs2value) >> reg.width()) as u32;

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u128 = u64::from(rs1v) as u128;
            let rs2value: u128 = u64::from(rs2v) as u128;
            let result: u64 = (rs1value.wrapping_mul(rs2value) >> reg.width()) as u64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            todo!();
        }
        _ => unreachable!(),
    }
}

pub fn mulhsu(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!(
        "mulhsu\t{},{},{}",
        reg.name(rd),
        reg.name(rs1),
        reg.name(rs2)
    );

    match reg.width() {
        32 => {
            let rs1value: i64 = i32::from(rs1v) as i64;
            let rs2value: i64 = u32::from(rs2v) as u64 as i64;
            let result: i32 = (rs1value.wrapping_mul(rs2value) >> reg.width()) as i32;

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i128 = i64::from(rs1v) as i128;
            let rs2value: i128 = u64::from(rs2v) as u128 as i128;
            let result: i64 = (rs1value.wrapping_mul(rs2value) >> reg.width()) as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            todo!();
        }
        _ => unreachable!(),
    }
}

pub fn mulw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("mulw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = (rs1value as i32).wrapping_mul(rs2value as i32) as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = rs1value.wrapping_mul(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn div(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("div\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    if rs2v == Uint::zero(reg.width()) {
        reg.set(rd, &Uint::ff_ff(reg.width()));
        return;
    }

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v.clone());
            let rs2value: i32 = i32::from(rs2v);

            if rs1value == (-1 << 31) && rs2value == -1 {
                reg.set(rd, &rs1v);
            } else {
                let result: i32 = rs1value.wrapping_div(rs2value);

                reg.set(rd, &Uint::from(result));
            }
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v.clone());
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == (-1 << 63) && rs2value == -1 {
                reg.set(rd, &rs1v);
            } else {
                let result: i64 = rs1value.wrapping_div(rs2value);
                reg.set(rd, &Uint::from(result));
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v.clone());
            let rs2value: i128 = i128::from(rs2v);
            
            if rs1value == (-1 << 127) && rs2value == -1 {
                reg.set(rd, &rs1v);
            } else {
                let result: i128 = rs1value.wrapping_div(rs2value);

                reg.set(rd, &Uint::from(result));
            }
        }
        _ => unreachable!(),
    }
}

pub fn divu(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("divu\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    if rs2v == Uint::zero(reg.width()) {
        reg.set(rd, &Uint::ff_ff(reg.width()));
        return;
    }

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let rs2value: u32 = u32::from(rs2v);
            let result: u32 = rs1value.wrapping_div(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let rs2value: u64 = u64::from(rs2v);

            let result: u64 = rs1value.wrapping_div(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let rs2value: u128 = u128::from(rs2v);

            let result: u128 = rs1value.wrapping_div(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn divw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("divw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    if rs2v == Uint::zero(reg.width()) {
        reg.set(rd, &Uint::ff_ff(reg.width()));
        return;
    }

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v.clone());
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == (-1 << 63) && rs2value == -1 {
                reg.set(rd, &rs1v);
            } else {
                let result: i64 = (rs1value as i32).wrapping_div(rs2value as i32) as i64;

                reg.set(rd, &Uint::from(result));
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v.clone());
            let rs2value: i128 = i128::from(rs2v);

            if rs1value == (-1 << 127) && rs2value == -1 {
                reg.set(rd, &rs1v);
            } else {
                let result: i128 = (rs1value as i32).wrapping_div(rs2value as i32) as i128;

                reg.set(rd, &Uint::from(result));
            }
        }
        _ => unreachable!(),
    }
}

pub fn divuw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!(
        "divuw\t{},{},{}",
        reg.name(rd),
        reg.name(rs1),
        reg.name(rs2)
    );

    if rs2v == Uint::zero(reg.width()) {
        reg.set(rd, &Uint::ff_ff(reg.width()));
        return;
    }

    match reg.width() {
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let rs2value: u64 = u64::from(rs2v);
            let result: i64 = (rs1value as u32).wrapping_div(rs2value as u32) as i32 as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let rs2value: u128 = u128::from(rs2v);
            let result: i128 = (rs1value as u32).wrapping_div(rs2value as u32) as i32 as i128;

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn rem(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("rem\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    if rs2v == Uint::zero(reg.width()) {
        reg.set(rd, &rs1v);
        return;
    }

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v.clone());
            let rs2value: i32 = i32::from(rs2v);

            if rs1value == (-1 << 31) && rs2value == -1 {
                reg.set(rd, &Uint::from(0u32));
            } else {
                let result: i32 = rs1value.wrapping_rem(rs2value);

                reg.set(rd, &Uint::from(result));
            }
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v.clone());
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == (-1 << 63) && rs2value == -1 {
                reg.set(rd, &Uint::from(0u64));
            } else {
                let result: i64 = rs1value.wrapping_rem(rs2value);
                reg.set(rd, &Uint::from(result));
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v.clone());
            let rs2value: i128 = i128::from(rs2v);

            if rs1value == (-1 << 127) && rs2value == -1 {
                reg.set(rd, &Uint::from(0u128));
            } else {
                let result: i128 = rs1value.wrapping_rem(rs2value);

                reg.set(rd, &Uint::from(result));
            }
        }
        _ => unreachable!(),
    }
}

pub fn remu(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("remu\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    if rs2v == Uint::zero(reg.width()) {
        reg.set(rd, &rs1v);
        return;
    }

    match reg.width() {
        32 => {
            let rs2value: u32 = u32::from(rs2v);
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value.wrapping_rem(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        64 => {
            let rs2value: u64 = u64::from(rs2v);
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value.wrapping_rem(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs2value: u128 = u128::from(rs2v);
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value.wrapping_rem(rs2value);

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn remw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!("remw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    if rs2v == Uint::zero(reg.width()) {
        reg.set(rd, &rs1v);
        return;
    }

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from(rs1v.clone());
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == (-1 << 63) && rs2value == -1 {
                reg.set(rd, &Uint::from(0u64));
            } else {
                let result: i64 = (rs1value as i32).wrapping_rem(rs2value as i32) as i64;

                reg.set(rd, &Uint::from(result));
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v.clone());
            let rs2value: i128 = i128::from(rs2v);

            if rs1value == (-1 << 127) && rs2value == -1 {
                reg.set(rd, &Uint::from(0u128));
            } else {
                let result: i128 = (rs1value as i32).wrapping_rem(rs2value as i32) as i128;

                reg.set(rd, &Uint::from(result));
            }
        }
        _ => unreachable!(),
    }
}

pub fn remuw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = reg.get(rs1);
    let rs2v: Uint = reg.get(rs2);

    println!(
        "remuw\t{},{},{}",
        reg.name(rd),
        reg.name(rs1),
        reg.name(rs2)
    );

    if rs2v == Uint::zero(reg.width()) {
        reg.set(rd, &rs1v);
        return;
    }

    match reg.width() {
        64 => {
            let rs2value: u64 = u64::from(rs2v);
            let rs1value: u64 = u64::from(rs1v);
            let result: i64 = (rs1value as u32).wrapping_rem(rs2value as u32) as i32 as i64;

            reg.set(rd, &Uint::from(result));
        }
        128 => {
            let rs2value: u128 = u128::from(rs2v);
            let rs1value: u128 = u128::from(rs1v);
            let result: i128 = (rs1value as u32).wrapping_rem(rs2value as u32) as i32 as i128;

            reg.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}
