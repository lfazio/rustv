use crate::vsoc::arch::riscv::{exception::RvException, registers::RvRegisters};

pub fn beq(x: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = x.get(rs1);
    let rs2v = x.get(rs2);

    if rs2 > 0 {
        print!("beq\t{},{},", x.name(rs1), x.name(rs2));
    } else {
        print!("beqz\t{},", x.name(rs1));
    }

    if rs1v == rs2v {
        return Ok(true);
    }

    Ok(false)
}

pub fn bne(x: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = x.get(rs1);
    let rs2v = x.get(rs2);

    if rs2 > 0 {
        print!("bne\t{},{},", x.name(rs1), x.name(rs2));
    } else {
        print!("bnez\t{},", x.name(rs1));
    }

    if rs1v != rs2v {
        return Ok(true);
    }

    Ok(false)
}

pub fn blt(x: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = x.get(rs1);
    let rs2v = x.get(rs2);

    if rs2 > 0 {
        print!("blt\t{},{},", x.name(rs1), x.name(rs2));
    } else {
        print!("bltz\t{},", x.name(rs1));
    }

    match x.len() {
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

pub fn bge(x: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = x.get(rs1);
    let rs2v = x.get(rs2);

    if rs2 > 0 {
        print!("bge\t{},{},", x.name(rs1), x.name(rs2));
    } else {
        print!("bgez\t{},", x.name(rs1));
    }

    match x.len() {
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

pub fn bltu(x: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = x.get(rs1);
    let rs2v = x.get(rs2);

    if rs2 > 0 {
        print!("bltu\t{},{},", x.name(rs1), x.name(rs2));
    } else {
        print!("bltuz\t{},", x.name(rs1));
    }

    if rs1v < rs2v {
        return Ok(true);
    }

    Ok(false)
}

pub fn bgeu(x: &mut RvRegisters, rs1: usize, rs2: usize) -> Result<bool, RvException> {
    let rs1v = x.get(rs1);
    let rs2v = x.get(rs2);

    if rs2 > 0 {
        print!("bgeu\t{},{},", x.name(rs1), x.name(rs2));
    } else {
        print!("bgeuz\t{},", x.name(rs1));
    }

    if rs1v >= rs2v {
        return Ok(true);
    }

    Ok(false)
}
