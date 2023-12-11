use crate::vsoc::arch::riscv::{exception::RvException, registers::RvRegisters};

pub fn beq(reg: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = reg.get(rs1);
    let rs2v = reg.get(rs2);

    if rs2 > 0 {
        print!("beq\t{},{},", reg.name(rs1), reg.name(rs2));
    } else {
        print!("beqz\t{},", reg.name(rs1));
    }

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);

            if rs1value == rs2value {
                return Ok(true);
            }
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);

            if rs1value == rs2value {
                return Ok(true);
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);

            if rs1value == rs2value {
                return Ok(true);
            }
        }
        _ => return Err(RvException::InstructionIllegal),
    }

    Ok(false)
}

pub fn bne(reg: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = reg.get(rs1);
    let rs2v = reg.get(rs2);

    if rs2 > 0 {
        print!("bne\t{},{},", reg.name(rs1), reg.name(rs2));
    } else {
        print!("bnez\t{},", reg.name(rs1));
    }

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);

            if rs1value != rs2value {
                return Ok(true);
            }
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);

            if rs1value != rs2value {
                return Ok(true);
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);

            if rs1value != rs2value {
                return Ok(true);
            }
        }
        _ => return Err(RvException::InstructionIllegal),
    }

    Ok(false)
}

pub fn blt(reg: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = reg.get(rs1);
    let rs2v = reg.get(rs2);

    if rs2 > 0 {
        print!("blt\t{},{},", reg.name(rs1), reg.name(rs2));
    } else {
        print!("bltz\t{},", reg.name(rs1));
    }

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);

            if rs1value < rs2value {
                return Ok(true);
            }
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);

            if rs1value < rs2value {
                return Ok(true);
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);

            if rs1value < rs2value {
                return Ok(true);
            }
        }
        _ => return Err(RvException::InstructionIllegal),
    }

    Ok(false)
}

pub fn bge(reg: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = reg.get(rs1);
    let rs2v = reg.get(rs2);

    if rs2 > 0 {
        print!("bge\t{},{},", reg.name(rs1), reg.name(rs2));
    } else {
        print!("bgez\t{},", reg.name(rs1));
    }

    match reg.width() {
        32 => {
            let rs1value: i32 = i32::from(rs1v);
            let rs2value: i32 = i32::from(rs2v);

            if rs1value >= rs2value {
                return Ok(true);
            }
        }
        64 => {
            let rs1value: i64 = i64::from(rs1v);
            let rs2value: i64 = i64::from(rs2v);

            if rs1value >= rs2value {
                return Ok(true);
            }
        }
        128 => {
            let rs1value: i128 = i128::from(rs1v);
            let rs2value: i128 = i128::from(rs2v);

            if rs1value >= rs2value {
                return Ok(true);
            }
        }
        _ => return Err(RvException::InstructionIllegal),
    }

    Ok(false)
}

pub fn bltu(reg: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = reg.get(rs1);
    let rs2v = reg.get(rs2);

    if rs2 > 0 {
        print!("bltu\t{},{},", reg.name(rs1), reg.name(rs2));
    } else {
        print!("bltuz\t{},", reg.name(rs1));
    }

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let rs2value: u32 = u32::from(rs2v);

            if rs1value < rs2value {
                return Ok(true);
            }
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let rs2value: u64 = u64::from(rs2v);

            if rs1value < rs2value {
                return Ok(true);
            }
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let rs2value: u128 = u128::from(rs2v);

            if rs1value < rs2value {
                return Ok(true);
            }
        }
        _ => return Err(RvException::InstructionIllegal),
    }

    Ok(false)
}

pub fn bgeu(reg: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = reg.get(rs1);
    let rs2v = reg.get(rs2);

    if rs2 > 0 {
        print!("bgeu\t{},{},", reg.name(rs1), reg.name(rs2));
    } else {
        print!("bgeuz\t{},", reg.name(rs1));
    }

    match reg.width() {
        32 => {
            let rs1value: u32 = u32::from(rs1v);
            let rs2value: u32 = u32::from(rs2v);

            if rs1value >= rs2value {
                return Ok(true);
            }
        }
        64 => {
            let rs1value: u64 = u64::from(rs1v);
            let rs2value: u64 = u64::from(rs2v);

            if rs1value >= rs2value {
                return Ok(true);
            }
        }
        128 => {
            let rs1value: u128 = u128::from(rs1v);
            let rs2value: u128 = u128::from(rs2v);

            if rs1value >= rs2value {
                return Ok(true);
            }
        }
        _ => return Err(RvException::InstructionIllegal),
    }

    Ok(false)
}
