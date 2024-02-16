use crate::vsoc::arch::types::Uint;

use super::super::registers::RvRegisters;

pub fn add(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("add\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);
            let result: i32 = rs1value.wrapping_add(rs2value);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = rs1value.wrapping_add(rs2value);

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = rs1value.wrapping_add(rs2value);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn addw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("addw\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = (rs1value as i32).wrapping_add(rs2value as i32) as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = (rs1value as i32).wrapping_add(rs2value as i32) as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sub(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("sub\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);
            let result: i32 = rs1value.wrapping_sub(rs2value);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = rs1value.wrapping_sub(rs2value);

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = rs1value.wrapping_sub(rs2value);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn subw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("subw\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = (rs1value as i32).wrapping_sub(rs2value as i32) as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = (rs1value as i32).wrapping_sub(rs2value as i32) as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn slt(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("slt\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);
            let result: i32 = if rs1value < rs2value { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = if rs1value < rs2value { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = if rs1value < rs2value { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sltu(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("sltu\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        32 => {
            let result: u32 = if rs1v < rs2v { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let result: u64 = if rs1v < rs2v { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let result: u128 = if rs1v < rs2v { 1 } else { 0 };

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn and(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);
    let result: Uint = rs1v & rs2v;

    println!("and\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    x.set(rd, &result);
}

pub fn or(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);
    let result: Uint = rs1v | rs2v;

    println!("or\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    x.set(rd, &result);
}

pub fn xor(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);
    let result: Uint = rs1v ^ rs2v;

    println!("xor\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    x.set(rd, &result);
}

pub fn sll(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let shamt: u32 = u32::from(x.get(rs2)) & 0x1f;

    println!("sll\t{},{},{}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value.wrapping_shl(shamt);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value.wrapping_shl(shamt);

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value.wrapping_shl(shamt);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sllw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let shamt: u32 = u32::from(x.get(rs2)) & 0x1f;

    println!("sllw\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: i64 = rs1value.wrapping_shl(shamt) as i32 as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: i128 = rs1value.wrapping_shl(shamt) as i32 as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn srl(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let shamt: u32 = u32::from(x.get(rs2)) & 0x1f;

    println!("srl\t{},{},{}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value.wrapping_shr(shamt);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value >> shamt;
            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value >> shamt;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn srlw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let shamt: u32 = u32::from(x.get(rs2)) & 0x1f;

    println!("srlw\t{},{},{}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = (rs1value as u32).wrapping_shr(shamt) as i32 as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = (rs1value as u32).wrapping_shr(shamt) as i32 as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sra(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let shamt: u32 = u32::from(x.get(rs2)) & 0x1f;

    println!("sra\t{},{},{}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let result: i32 = rs1value.wrapping_shr(shamt);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = rs1value.wrapping_shr(shamt);

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = rs1value.wrapping_shr(shamt);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn sraw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let shamt: u32 = u32::from(x.get(rs2)) & 0x1f;

    println!("sraw\t{},{},{}", x.name(rd), x.name(rs1), shamt);

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let result: i64 = (rs1value as i32 >> shamt) as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let result: i128 = (rs1value as i32 >> shamt) as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn mul(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("mul\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);
            let result: i32 = rs1value.wrapping_mul(rs2value);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = rs1value.wrapping_mul(rs2value);

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = rs1value.wrapping_mul(rs2value);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn mulh(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("mulh\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        32 => {
            let rs1value: i64 = i32::from(rs1v) as i64;
            let rs2value: i64 = i32::from(rs2v) as i64;
            let result: i32 = (rs1value.wrapping_mul(rs2value) >> x.len()) as i32;

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i128 = i64::from(rs1v) as i128;
            let rs2value: i128 = i64::from(rs2v) as i128;
            let result: i64 = (rs1value.wrapping_mul(rs2value) >> x.len()) as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            todo!();
        }
        _ => unreachable!(),
    }
}

pub fn mulhu(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!(
        "mulhu\t{},{},{}",
        x.name(rd),
        x.name(rs1),
        x.name(rs2)
    );

    match x.len() {
        32 => {
            let rs1value: u64 = u32::from(rs1v) as u64;
            let rs2value: u64 = u32::from(rs2v) as u64;
            let result: u32 = (rs1value.wrapping_mul(rs2value) >> x.len()) as u32;

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u128 = u64::from(rs1v) as u128;
            let rs2value: u128 = u64::from(rs2v) as u128;
            let result: u64 = (rs1value.wrapping_mul(rs2value) >> x.len()) as u64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            todo!();
        }
        _ => unreachable!(),
    }
}

pub fn mulhsu(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!(
        "mulhsu\t{},{},{}",
        x.name(rd),
        x.name(rs1),
        x.name(rs2)
    );

    match x.len() {
        32 => {
            let rs1value: i64 = i32::from(rs1v) as i64;
            let rs2value: i64 = u32::from(rs2v) as u64 as i64;
            let result: i32 = (rs1value.wrapping_mul(rs2value) >> x.len()) as i32;

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: i128 = i64::from(rs1v) as i128;
            let rs2value: i128 = u64::from(rs2v) as u128 as i128;
            let result: i64 = (rs1value.wrapping_mul(rs2value) >> x.len()) as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            todo!();
        }
        _ => unreachable!(),
    }
}

pub fn mulw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("mulw\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);
            let result: i64 = (rs1value as i32).wrapping_mul(rs2value as i32) as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);
            let result: i128 = rs1value.wrapping_mul(rs2value);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn div(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("div\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    if rs2v == Uint::zero(x.len()) {
        x.set(rd, &Uint::ff_ff(x.len()));
        return;
    }

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v.clone());
            let rs2value: i32 = i32::from(rs2v);

            if rs1value == (-1 << 31) && rs2value == -1 {
                x.set(rd, &rs1v);
            } else {
                let result: i32 = rs1value.wrapping_div(rs2value);

                x.set(rd, &Uint::from(result));
            }
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v.clone());
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == (-1 << 63) && rs2value == -1 {
                x.set(rd, &rs1v);
            } else {
                let result: i64 = rs1value.wrapping_div(rs2value);
                x.set(rd, &Uint::from(result));
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v.clone());
            let rs2value: i128 = i128::from(rs2v);
            
            if rs1value == (-1 << 127) && rs2value == -1 {
                x.set(rd, &rs1v);
            } else {
                let result: i128 = rs1value.wrapping_div(rs2value);

                x.set(rd, &Uint::from(result));
            }
        }
        _ => unreachable!(),
    }
}

pub fn divu(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("divu\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    if rs2v == Uint::zero(x.len()) {
        x.set(rd, &Uint::ff_ff(x.len()));
        return;
    }

    match x.len() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let rs2value: u32 = u32::from(rs2v);
            let result: u32 = rs1value.wrapping_div(rs2value);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let rs2value: u64 = u64::from(rs2v);

            let result: u64 = rs1value.wrapping_div(rs2value);

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let rs2value: u128 = u128::from(rs2v);

            let result: u128 = rs1value.wrapping_div(rs2value);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn divw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("divw\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    if rs2v == Uint::zero(x.len()) {
        x.set(rd, &Uint::ff_ff(x.len()));
        return;
    }

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v.clone());
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == (-1 << 63) && rs2value == -1 {
                x.set(rd, &rs1v);
            } else {
                let result: i64 = (rs1value as i32).wrapping_div(rs2value as i32) as i64;

                x.set(rd, &Uint::from(result));
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v.clone());
            let rs2value: i128 = i128::from(rs2v);

            if rs1value == (-1 << 127) && rs2value == -1 {
                x.set(rd, &rs1v);
            } else {
                let result: i128 = (rs1value as i32).wrapping_div(rs2value as i32) as i128;

                x.set(rd, &Uint::from(result));
            }
        }
        _ => unreachable!(),
    }
}

pub fn divuw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!(
        "divuw\t{},{},{}",
        x.name(rd),
        x.name(rs1),
        x.name(rs2)
    );

    if rs2v == Uint::zero(x.len()) {
        x.set(rd, &Uint::ff_ff(x.len()));
        return;
    }

    match x.len() {
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let rs2value: u64 = u64::from(rs2v);
            let result: i64 = (rs1value as u32).wrapping_div(rs2value as u32) as i32 as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let rs2value: u128 = u128::from(rs2v);
            let result: i128 = (rs1value as u32).wrapping_div(rs2value as u32) as i32 as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn rem(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("rem\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    if rs2v == Uint::zero(x.len()) {
        x.set(rd, &rs1v);
        return;
    }

    match x.len() {
        32 => {
            let rs1value: i32 = i32::from(rs1v.clone());
            let rs2value: i32 = i32::from(rs2v);

            if rs1value == (-1 << 31) && rs2value == -1 {
                x.set(rd, &Uint::from(0u32));
            } else {
                let result: i32 = rs1value.wrapping_rem(rs2value);

                x.set(rd, &Uint::from(result));
            }
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v.clone());
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == (-1 << 63) && rs2value == -1 {
                x.set(rd, &Uint::from(0u64));
            } else {
                let result: i64 = rs1value.wrapping_rem(rs2value);
                x.set(rd, &Uint::from(result));
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v.clone());
            let rs2value: i128 = i128::from(rs2v);

            if rs1value == (-1 << 127) && rs2value == -1 {
                x.set(rd, &Uint::from(0u128));
            } else {
                let result: i128 = rs1value.wrapping_rem(rs2value);

                x.set(rd, &Uint::from(result));
            }
        }
        _ => unreachable!(),
    }
}

pub fn remu(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("remu\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    if rs2v == Uint::zero(x.len()) {
        x.set(rd, &rs1v);
        return;
    }

    match x.len() {
        32 => {
            let rs2value: u32 = u32::from(rs2v);
            let rs1value: u32 = u32::from(rs1v);
            let result: u32 = rs1value.wrapping_rem(rs2value);

            x.set(rd, &Uint::from(result));
        }
        64 => {
            let rs2value: u64 = u64::from(rs2v);
            let rs1value: u64 = u64::from(rs1v);
            let result: u64 = rs1value.wrapping_rem(rs2value);

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs2value: u128 = u128::from(rs2v);
            let rs1value: u128 = u128::from(rs1v);
            let result: u128 = rs1value.wrapping_rem(rs2value);

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}

pub fn remw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!("remw\t{},{},{}", x.name(rd), x.name(rs1), x.name(rs2));

    if rs2v == Uint::zero(x.len()) {
        x.set(rd, &rs1v);
        return;
    }

    match x.len() {
        64 => {
            let rs1value: i64 = i64::from(rs1v.clone());
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == (-1 << 63) && rs2value == -1 {
                x.set(rd, &Uint::from(0u64));
            } else {
                let result: i64 = (rs1value as i32).wrapping_rem(rs2value as i32) as i64;

                x.set(rd, &Uint::from(result));
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v.clone());
            let rs2value: i128 = i128::from(rs2v);

            if rs1value == (-1 << 127) && rs2value == -1 {
                x.set(rd, &Uint::from(0u128));
            } else {
                let result: i128 = (rs1value as i32).wrapping_rem(rs2value as i32) as i128;

                x.set(rd, &Uint::from(result));
            }
        }
        _ => unreachable!(),
    }
}

pub fn remuw(x: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Uint = x.get(rs1);
    let rs2v: Uint = x.get(rs2);

    println!(
        "remuw\t{},{},{}",
        x.name(rd),
        x.name(rs1),
        x.name(rs2)
    );

    if rs2v == Uint::zero(x.len()) {
        x.set(rd, &rs1v);
        return;
    }

    match x.len() {
        64 => {
            let rs2value: u64 = u64::from(rs2v);
            let rs1value: u64 = u64::from(rs1v);
            let result: i64 = (rs1value as u32).wrapping_rem(rs2value as u32) as i32 as i64;

            x.set(rd, &Uint::from(result));
        }
        128 => {
            let rs2value: u128 = u128::from(rs2v);
            let rs1value: u128 = u128::from(rs1v);
            let result: i128 = (rs1value as u32).wrapping_rem(rs2value as u32) as i32 as i128;

            x.set(rd, &Uint::from(result));
        }
        _ => unreachable!(),
    }
}
