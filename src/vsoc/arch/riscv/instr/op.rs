use super::super::registers::RvRegisters;

pub fn add(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("add\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));
    
    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i32 = i32::from_le_bytes(rs2v.try_into().unwrap());
            let result: i32 = rs1value + rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i64 = i64::from_le_bytes(rs2v.try_into().unwrap());
            let result: i64 = rs1value + rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i128 = i128::from_le_bytes(rs2v.try_into().unwrap());
            let result: i128 = rs1value + rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn addw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("addw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));
    
    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i64 = i64::from_le_bytes(rs2v.try_into().unwrap());
            let result: i64 = ((rs1value + rs2value) & 0xffffffff) as i32 as i64;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i128 = i128::from_le_bytes(rs2v.try_into().unwrap());
            let result: i128 = ((rs1value + rs2value) & 0xffffffff) as i32 as i128;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn sub(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("sub\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));
    
    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i32 = i32::from_le_bytes(rs2v.try_into().unwrap());
            let result: i32 = rs1value - rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i64 = i64::from_le_bytes(rs2v.try_into().unwrap());
            let result: i64 = rs1value - rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i128 = i128::from_le_bytes(rs2v.try_into().unwrap());
            let result: i128 = rs1value - rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn subw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("subw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));
    
    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i64 = i64::from_le_bytes(rs2v.try_into().unwrap());
            let result: i64 = ((rs1value - rs2value) & 0xffffffff) as i32 as i64;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i128 = i128::from_le_bytes(rs2v.try_into().unwrap());
            let result: i128 = ((rs1value - rs2value) & 0xffffffff) as i32 as i128;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn slt(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("slt\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i32 = i32::from_le_bytes(rs2v.try_into().unwrap());
            let result: i32 = if rs1value < rs2value { 1 } else { 0 };

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i64 = i64::from_le_bytes(rs2v.try_into().unwrap());
            let result: i64 = if rs1value < rs2value as i64 { 1 } else { 0 };

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i128 = i128::from_le_bytes(rs2v.try_into().unwrap());
            let result: i128 = if rs1value < rs2value as i128 { 1 } else { 0 };

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn sltu(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("sltu\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: u32 = u32::from_le_bytes(rs2v.try_into().unwrap());
            let result: u32 = if rs1value < rs2value { 1 } else { 0 };

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: u64 = u64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: u64 = u64::from_le_bytes(rs2v.try_into().unwrap());
            let result: u64 = if rs1value < rs2value as u64 { 1 } else { 0 };

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: u128 = u128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: u128 = u128::from_le_bytes(rs2v.try_into().unwrap());
            let result: u128 = if rs1value < rs2value as u128 { 1 } else { 0 };

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn and(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("and\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i32 = i32::from_le_bytes(rs2v.try_into().unwrap());
            let result: i32 = rs1value & rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i64 = i64::from_le_bytes(rs2v.try_into().unwrap());
            let result: i64 = rs1value & rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i128 = i128::from_le_bytes(rs2v.try_into().unwrap());
            let result: i128 = rs1value & rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn or(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("or\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i32 = i32::from_le_bytes(rs2v.try_into().unwrap());
            let result: i32 = rs1value | rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i64 = i64::from_le_bytes(rs2v.try_into().unwrap());
            let result: i64 = rs1value | rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i128 = i128::from_le_bytes(rs2v.try_into().unwrap());
            let result: i128 = rs1value | rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn xor(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let rs2v: Vec<u8> = reg.get(rs2);

    println!("xor\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i32 = i32::from_le_bytes(rs2v.try_into().unwrap());
            let result: i32 = rs1value ^ rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i64 = i64::from_le_bytes(rs2v.try_into().unwrap());
            let result: i64 = rs1value ^ rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let rs2value: i128 = i128::from_le_bytes(rs2v.try_into().unwrap());
            let result: i128 = rs1value ^ rs2value;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn sll(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let shamt: u32 = reg.get(rs2)[0] as u32;

    println!("sll\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from_le_bytes(rs1v.try_into().unwrap());
            let result: u32 = rs1value.wrapping_shl(shamt);

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: u64 = u64::from_le_bytes(rs1v.try_into().unwrap());
            let result: u64 = rs1value.wrapping_shl(shamt);

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: u128 = u128::from_le_bytes(rs1v.try_into().unwrap());
            let result: u128 = rs1value.wrapping_shl(shamt);

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn sllw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let shamt: u32 = reg.get(rs2)[0] as u32 & 0x1f;

    println!("sllw\t{},{},{}", reg.name(rd), reg.name(rs1), reg.name(rs2));

    match reg.width() {
        64 => {
            let rs1value: u64 = u64::from_le_bytes(rs1v.try_into().unwrap());
            let result: i64 = rs1value.wrapping_shl(shamt) as i32 as i64;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: u128 = u128::from_le_bytes(rs1v.try_into().unwrap());
            let result: i128 = rs1value.wrapping_shl(shamt) as i32 as i128;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn srl(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let shamt: u32 = reg.get(rs2)[0] as u32 & 0x1f;

    println!("srl\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from_le_bytes(rs1v.try_into().unwrap());
            let result: u32 = rs1value.wrapping_shr(shamt);

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: u64 = u64::from_le_bytes(rs1v.try_into().unwrap());
            let result: u64 = rs1value >> shamt;
            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: u128 = u128::from_le_bytes(rs1v.try_into().unwrap());
            let result: u128 = rs1value >> shamt;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn srlw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let shamt: u32 = reg.get(rs2)[0] as u32 & 0x1f;

    println!("srlw\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let result: i64 = (rs1value as u32).wrapping_shr(shamt as u32) as i32 as i64;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let result: i128 = (rs1value as u32).wrapping_shr(shamt as u32) as i32 as i128;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn sra(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let shamt: u32 = reg.get(rs2)[0] as u32 & 0x1f;

    println!("sra\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from_le_bytes(rs1v.try_into().unwrap());
            let result: i32 = rs1value.wrapping_shr(shamt);

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let result: i64 = rs1value.wrapping_shr(shamt);

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let result: i128 = rs1value.wrapping_shr(shamt);

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}

pub fn sraw(reg: &mut RvRegisters, rd: usize, rs1: usize, rs2: usize) {
    let rs1v: Vec<u8> = reg.get(rs1);
    let shamt: u32 = reg.get(rs2)[0] as u32 & 0x1f;

    println!("sraw\t{},{},{}", reg.name(rd), reg.name(rs1), shamt);

    match reg.width() {
        64 => {
            let rs1value: i64 = i64::from_le_bytes(rs1v.try_into().unwrap());
            let result: i64 = (rs1value as i32 >> shamt) as i64;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        128 => {
            let rs1value: i128 = i128::from_le_bytes(rs1v.try_into().unwrap());
            let result: i128 = (rs1value as i32 >> shamt) as i128;

            reg.set(rd, result.to_le_bytes().as_slice());
        },
        _ => unreachable!(),
    }
}
