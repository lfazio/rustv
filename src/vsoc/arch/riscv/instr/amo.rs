use crate::vsoc::arch::riscv::atomic::AtomicCtx;
use crate::vsoc::arch::riscv::registers::RvRegisters;
use crate::vsoc::{
    arch::{riscv::exception::RvException, types::Uint},
    bus::Bus,
};

pub fn lr(
    ctx: &mut AtomicCtx,
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };

    if rs2 != 0 {
        return Some(RvException::InstructionIllegal);
    }

    let addr: u64 = u64::from(x.get(rs1));
    ctx.reserve(addr);

    let mut value: Uint = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v),
        Err(e) => return Some(RvException::from(e)),
    };
    if (width == 32) && x.len() > 32 {
        value.sextend(x.len(), width);
    }
    x.set(rd, &value);

    println!(
        "lr.{}\t{},({})\t# {:0x}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs1),
        addr
    );

    None
}

pub fn sc(
    ctx: &mut AtomicCtx,
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value: Uint = x.get(rs2);

    if ctx.check(addr) {
        match bus.store(width / 8, addr, &Vec::<u8>::from(value.clone())) {
            None => (),
            Some(e) => return Some(RvException::from(e)),
        }

        x.set(rd, &Uint::zero(x.len()));
    } else {
        x.set(rd, &Uint::one(x.len()));
    }

    ctx.release();

    println!(
        "sc.{}\t{},{},({})\t# {:0x}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs2),
        x.name(rs1),
        addr
    );

    None
}

pub fn swap(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let rs2val: Uint = x.get(rs2);
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };

    x.set(rd, &value);

    match bus.store(width / 8, addr, &Vec::<u8>::from(rs2val.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!(
        "amoswap.{}\t{},{},({})\t# {:0x}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs2),
        x.name(rs1),
        addr
    );

    None
}

pub fn add(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };

    x.set(rd, &value);
    let mut result = value + x.get(rs2);

    match bus.store(width / 8, addr, &Vec::<u8>::from(result.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!(
        "amoadd.{}\t{},{},({})\t# @{:0x}={}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs2),
        x.name(rs1),
        addr,
        result.truncate(width)
    );

    None
}

pub fn xor(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };

    x.set(rd, &value);
    let mut result = value ^ x.get(rs2);

    match bus.store(width / 8, addr, &Vec::<u8>::from(result.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!(
        "amoxor.{}\t{},{},({})\t# @{:0x}={}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs2),
        x.name(rs1),
        addr,
        result.truncate(width)
    );

    None
}

pub fn and(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };

    x.set(rd, &value);
    let mut result = value & x.get(rs2);

    match bus.store(width / 8, addr, &Vec::<u8>::from(result.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!(
        "amoand.{}\t{},{},({})\t# @{:0x}={}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs2),
        x.name(rs1),
        addr,
        result.truncate(width)
    );

    None
}

pub fn or(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };

    x.set(rd, &value);
    let mut result = value | x.get(rs2);

    match bus.store(width / 8, addr, &Vec::<u8>::from(result.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!(
        "amoor.{}\t{},{},({})\t# @{:0x}={}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs2),
        x.name(rs1),
        addr,
        result.truncate(width)
    );

    None
}

pub fn min(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };
    let result: Uint;

    x.set(rd, &value.clone());
    match width {
        32 => {
            let rd_val: i32 = i32::from(value.clone());
            let rs2_val: i32 = i32::from(x.get(rs2));
            result = if rd_val < rs2_val {
                value.clone()
            } else {
                x.get(rs2)
            };

            print!(
                "amomin.{}\t{},{},({})\t# min({}, {}) => @{:0x}",
                if width == 32 { "w" } else { "d" },
                x.name(rd),
                x.name(rs2),
                x.name(rs1),
                rd_val,
                rs2_val,
                addr
            );
        }
        64 => {
            let rd_val: i64 = i64::from(value.clone());
            let rs2_val: i64 = i64::from(x.get(rs2));
            result = if rd_val < rs2_val {
                value.clone()
            } else {
                x.get(rs2)
            };

            print!(
                "amomin.{}\t{},{},({})\t# min({}, {}) => @{:0x}",
                if width == 32 { "w" } else { "d" },
                x.name(rd),
                x.name(rs2),
                x.name(rs1),
                rd_val,
                rs2_val,
                addr
            );
        }
        _ => unreachable!(),
    };

    match bus.store(width / 8, addr, &Vec::<u8>::from(result.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!("{}", result);

    None
}

pub fn minu(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };

    x.set(rd, &value);
    let result = if value < x.get(rs2) {
        value.clone()
    } else {
        x.get(rs2)
    };

    match bus.store(width / 8, addr, &Vec::<u8>::from(result.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!(
        "amominu.{}\t{},{},({})\t# minu({}, {}) => @{:0x}={}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs2),
        x.name(rs1),
        value,
        x.get(rs2),
        addr,
        result
    );

    None
}

pub fn max(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };
    let result: Uint;

    x.set(rd, &value);
    match width {
        32 => {
            let rd_val: i32 = i32::from(value.clone());
            let rs2_val: i32 = i32::from(x.get(rs2));
            result = if rd_val > rs2_val {
                value.clone()
            } else {
                x.get(rs2)
            };

            print!(
                "amomax.{}\t{},{},({})\t# max({}, {}) => @{:0x}=",
                if width == 32 { "w" } else { "d" },
                x.name(rd),
                x.name(rs2),
                x.name(rs1),
                rd_val,
                rs2_val,
                addr
            );
        }
        64 => {
            let rd_val: i64 = i64::from(value.clone());
            let rs2_val: i64 = i64::from(x.get(rs2));
            result = if rd_val > rs2_val {
                value.clone()
            } else {
                x.get(rs2)
            };

            print!(
                "amomax.{}\t{},{},({})\t# max({}, {}) => @{:0x}=",
                if width == 32 { "w" } else { "d" },
                x.name(rd),
                x.name(rs2),
                x.name(rs1),
                rd_val,
                rs2_val,
                addr
            );
        }
        _ => unreachable!(),
    };

    match bus.store(width / 8, addr, &Vec::<u8>::from(result.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!("{}", result);

    None
}

pub fn maxu(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let value = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };

    x.set(rd, &value);
    let result = if value > x.get(rs2) {
        value.clone()
    } else {
        x.get(rs2)
    };

    match bus.store(width / 8, addr, &Vec::<u8>::from(result.clone())) {
        None => (),
        Some(e) => return Some(RvException::from(e)),
    }

    println!(
        "amomaxu.{}\t{},{},({})\t# maxu({}, {}) => @{:0x}={}",
        if width == 32 { "w" } else { "d" },
        x.name(rd),
        x.name(rs2),
        x.name(rs1),
        value,
        x.get(rs2),
        addr,
        result
    );

    None
}

pub fn cas(
    _aq: bool,
    _rl: bool,
    funct3: usize,
    x: &mut RvRegisters,
    rd: usize,
    rs1: usize,
    rs2: usize,
    bus: &mut Bus,
) -> Option<RvException> {
    let width: usize = match funct3 {
        2 => 32,
        3 => 64,
        4 => 128,
        _ => unreachable!(),
    };
    let addr: u64 = u64::from(x.get(rs1));
    let result: Uint;

    if width > 2 * x.len() {
        return Some(RvException::InstructionIllegal);
    }

    let temp0 = match bus.fetch(width / 8, addr) {
        Ok(v) => Uint::new(v).sextend(x.len(), width),
        Err(e) => return Some(RvException::from(e)),
    };
    let comp0 = x.get(rd);
    let swap0 = x.get(rs2);

    let mut temp1: Uint = Uint::zero(x.len());
    let mut comp1: Uint = Uint::zero(x.len());
    let mut swap1: Uint = Uint::zero(x.len());

    if width == 2 * x.len() {
        temp1 = match bus.fetch(width / 8, addr + (x.len() / 8) as u64) {
            Ok(v) => Uint::new(v).sextend(x.len(), width),
            Err(e) => return Some(RvException::from(e)),
        };
        comp1 = x.get(rd + 1);
        swap1 = x.get(rs2 + 1);
    };

    if temp0 == comp0 && temp1 == comp1 {
        match bus.store(x.len() / 8, addr, &Vec::<u8>::from(swap0)) {
            None => (),
            Some(e) => return Some(RvException::from(e)),
        }

        if width == 2 * x.len() {
            match bus.store(
                x.len() / 8,
                addr + (x.len() / 8) as u64,
                &Vec::<u8>::from(swap1),
            ) {
                None => (),
                Some(e) => return Some(RvException::from(e)),
            }
        }
    }

    if rd != rs1 {
        x.set(rd, &temp0);
        if width == 2 * x.len() {
            x.set(rd + 1, &temp1);
        }
    }

    println!(
        "amocas.{}\t{},{},({})",
        if width == 32 {
            "w"
        } else if width == 64 {
            "d"
        } else {
            "q"
        },
        x.name(rd),
        x.name(rs2),
        x.name(rs1)
    );

    None
}
