mod auipc;
mod lui;
mod branch;
mod load;
mod op;
mod opimm;
mod store;
mod system;
mod todo;

use super::csr::Csr;
use super::exception;
use super::registers::RvRegisters;
use super::hart::Rv;
use crate::vsoc::arch::types::Uint;
use crate::vsoc::bus::Bus;

pub enum Instr {
    Invalid,
    InstrC0(u16),
    InstrC1(u16),
    InstrC2(u16),
    Instr32(u32),
}

impl From<Vec<u8>> for Instr {
    fn from(v: Vec<u8>) -> Self {
        if v.len() != 4 {
            return Instr::Invalid;
        }

        Instr::Instr32(u32::from_le_bytes(v.try_into().unwrap()))
    }
}

impl Instr {
    pub fn new(raw_instr: u32) -> Instr {
        match raw_instr & 0x3 {
            0x3 => {
                if ((raw_instr >> 2) & 0x7) == 0x07 {
                    return Instr::Invalid;
                }

                Instr::Instr32(raw_instr)
            },
            0x2 => Instr::InstrC2((raw_instr & 0xffff).try_into().unwrap()),
            0x1 => Instr::InstrC1((raw_instr & 0xffff).try_into().unwrap()),
            0x0 => Instr::InstrC0((raw_instr & 0xffff).try_into().unwrap()),
            _ => Instr::Invalid,
        }
    }

    pub fn get_raw(&self) -> u32 {
        match self {
            Instr::InstrC0(v) => *v as u32,
            Instr::InstrC1(v) => *v as u32,
            Instr::InstrC2(v) => *v as u32,
            Instr::Instr32(v) => *v,
            _ => unreachable!(),
        }
    }

    fn load(&self, reg: &mut RvRegisters, bus: &mut Bus) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let imm: i32 = self.get_i_imm();

        let result = match funct3 {
            0x0 => load::lb(reg, rd, rs1, imm, bus), // load byte
            0x1 => load::lh(reg, rd, rs1, imm, bus), // load half
            0x2 => load::lw(reg, rd, rs1, imm, bus), // load word
            0x3 => load::ld(reg, rd, rs1, imm, bus), // load double
            0x4 => load::lbu(reg, rd, rs1, imm, bus), // load byte unsigned
            0x5 => load::lhu(reg, rd, rs1, imm, bus), // load half unsigned
            0x6 => load::lwu(reg, rd, rs1, imm, bus), // load word unsigned
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        match result {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    fn op(&self, reg: &mut RvRegisters) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let funct7: usize = self.get_funct7();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();

        match self.get_funct7() {
            0x01 => match funct3 {
                0x0 => op::mul(reg, rd, rs1, rs2),
                0x1 => op::mulh(reg, rd, rs1, rs2),
                0x2 => op::mulhsu(reg, rd, rs1, rs2),
                0x3 => op::mulhu(reg, rd, rs1, rs2),
                0x4 => op::div(reg, rd, rs1, rs2),
                0x5 => op::divu(reg, rd, rs1, rs2),
                0x6 => op::rem(reg, rd, rs1, rs2),
                0x7 => op::remu(reg, rd, rs1, rs2),
                _ => return Some(exception::RvException::InstructionIllegal),
            },
            _ => match funct3 {
                0x0 => match self.get_funct7() {
                    0x00 => op::add(reg, rd, rs1, rs2), // add
                    0x20 => op::sub(reg, rd, rs1, rs2), // sub
                    _ => return Some(exception::RvException::InstructionIllegal),
                },
                0x1 => op::sll(reg, rd, rs1, rs2), // sll
                0x2 => op::slt(reg, rd, rs1, rs2), // slt
                0x3 => op::sltu(reg, rd, rs1, rs2), // sltu
                0x4 => op::xor(reg, rd, rs1, rs2), // xor
                0x5 => if (funct7 & 0x20) == 0x20 {
                    op::sra(reg, rd, rs1, rs2); // sra
                } else {
                    op::srl(reg, rd, rs1, rs2); // srl
                },
                0x6 => op::or(reg, rd, rs1, rs2), // or
                0x7 => op::and(reg, rd, rs1, rs2), // and
                _ => return Some(exception::RvException::InstructionIllegal),
            },
        };

        None
    }

    fn op32(&self, reg: &mut RvRegisters) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();

        match self.get_funct7() {
            0x01 => match funct3 {
                0x0 => op::mulw(reg, rd, rs1, rs2),
                0x4 => op::divw(reg, rd, rs1, rs2),
                0x5 => op::divuw(reg, rd, rs1, rs2),
                0x6 => op::remw(reg, rd, rs1, rs2),
                0x7 => op::remuw(reg, rd, rs1, rs2),
                _ => return Some(exception::RvException::InstructionIllegal),
            },
            _ => match funct3 {
                0x0 => match self.get_funct7() {
                    0x00 => op::addw(reg, rd, rs1, rs2), // addw
                    0x20 => op::subw(reg, rd, rs1, rs2), // subw
                    _ => return Some(exception::RvException::InstructionIllegal),
                },
                0x1 => op::sllw(reg, rd, rs1, rs2), // sllw
                0x5 => match self.get_funct7() {
                    0x00 => op::srlw(reg, rd, rs1, rs2), // srlw
                    0x20 => op::sraw(reg, rd, rs1, rs2), // sraw
                    _ => return Some(exception::RvException::InstructionIllegal),
                },
                _ => return Some(exception::RvException::InstructionIllegal),
            },
        };

        None
    }

    fn mem(&self) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let fm: usize = self.get_imm(31, 28) as usize >> 28;

        match funct3 {
            0x0 => match fm {
                0x0 => println!("fence"),
                0x8 => println!("fence.tso"),
                _ => return Some(exception::RvException::InstructionIllegal),
            },
            0x1 => println!("fence.i"),
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        None
    }

    fn opimm(&self, reg: &mut RvRegisters) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let funct7: usize = self.get_funct7();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let imm: i32 = self.get_i_imm();
        let shamt: usize = self.get_shamt();

        match funct3 {
            0x0 => opimm::addi(reg, rd, rs1, imm), // addi
            0x1 => opimm::slli(reg, rd, rs1, shamt), // slli
            0x2 => opimm::slti(reg, rd, rs1, imm), // slti
            0x3 => opimm::sltiu(reg, rd, rs1, imm), // sltiu
            0x4 => opimm::xori(reg, rd, rs1, imm), // xori
            0x5 => if funct7 & 0x20 == 0x20 {
                opimm::srai(reg, rd, rs1, shamt); // srai
            } else {
                opimm::srli(reg, rd, rs1, shamt); // srli
            },
            0x6 => opimm::ori(reg, rd, rs1, imm), // ori
            0x7 => opimm::andi(reg, rd, rs1, imm), // andi
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        None
    }

    fn opimm32(&self, reg: &mut RvRegisters) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let funct7: usize = self.get_funct7();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();
        let imm: i32 = self.get_i_imm();

        match funct3 {
            0x0 => opimm::addiw(reg, rd, rs1, imm), // addiw
            0x1 => opimm::slliw(reg, rd, rs1, rs2), // slliw
            0x5 => if funct7 & 0x20 == 0x20 {
                opimm::sraiw(reg, rd, rs1, rs2); // sraiw
            } else {
                opimm::srliw(reg, rd, rs1, rs2); // srliw
            },
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        None
    }

    fn auipc(&self, reg: &mut RvRegisters, pc: &Uint) -> Option<exception::RvException> {
        let rd: usize = self.get_rd();
        let imm: i32 = self.get_u_imm();

        auipc::auipc(reg, rd, pc, imm);

        None
    }

    fn lui(&self, reg: &mut RvRegisters) -> Option<exception::RvException> {
        let rd: usize = self.get_rd();
        let imm: i32 = self.get_u_imm();

        lui::lui(reg, rd, imm);

        None
    }

    fn jalr(&self, reg: &mut RvRegisters, pc: &Uint) -> Result<i128, exception::RvException> {
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let imm: i32 = self.get_i_imm();
        let v: u128;
        let offset: i128;

        match reg.width() {
            32 => {
                let base: i32 = i32::from(reg.get(rs1));
                let value: i32 = i32::from(pc.clone());

                v = (base as u128 + 4) & 0xffffffff;
                reg.set(rd, &Uint::from(value + 4));
                offset = (base - value) as i128 + imm as i128;
            },
            64 => {
                let base: i64 = i64::from(reg.get(rs1));
                let value: i64 = i64::from(pc.clone());

                v = (base as u128 + 4) & 0xffffffffffffffff;
                reg.set(rd, &Uint::from(value + 4));
                offset = (base - value) as i128 + imm as i128;
            },
            128 => {
                let base: i128 = i128::from(reg.get(rs1));
                let value: i128 = i128::from(pc.clone());

                v = base as u128 + 4;
                reg.set(rd, &Uint::from(value + 4));
                offset = base - value + imm as i128;
            },
            _ => unreachable!(),
        }

        println!("jalr\t{},{:0x}", reg.name(rd), v);

        Ok(offset)
    }

    fn jal(&self, reg: &mut RvRegisters, pc: &Uint) -> Result<i128, exception::RvException> {
        let rd: usize = self.get_rd();
        let imm: i32 = self.get_j_imm();
        let v: u128;

        match reg.width() {
            32 => {
                let value: u32 = u32::from(pc.clone()) + 4;

                v = value as u128;
                reg.set(rd, &Uint::from(value));
            },
            64 => {
                let value: u64 = u64::from(pc.clone()) + 4;

                v = value as u128;
                reg.set(rd, &Uint::from(value));

            },
            128 => {
                let value: u128 = u128::from(pc.clone()) + 4;

                v = value;
                reg.set(rd, &Uint::from(value));

            },
            _ => unreachable!(),
        };

        if rd == 0 {
            println!("j\t{:0x}", v);
        } else {
            println!("jal\t{},{:0x}", reg.name(rd), v);
        }

        Ok(imm as i128)
    }

    fn branch(&self, reg: &mut RvRegisters, pc: &Uint) -> Result<i128, exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();
        let offset: i32 = self.get_b_imm();
        let branched: bool = match funct3 {
            0x0 => branch::beq(reg, rs1, rs2).unwrap(), // beq
            0x1 => branch::bne(reg, rs1, rs2).unwrap(), // bne
            0x4 => branch::blt(reg, rs1, rs2).unwrap(), // blt
            0x5 => branch::bge(reg, rs1, rs2).unwrap(), // bge
            0x6 => branch::bltu(reg, rs1, rs2).unwrap(), // bltu
            0x7 => branch::bgeu(reg, rs1, rs2).unwrap(), // bgeu
            _ => return Err(exception::RvException::InstructionIllegal),
        };

        match reg.width() {
            32 => {
                let pcvalue: i32 = i32::from(pc.clone()) + offset;

                println!("{:0x}", pcvalue)
            },
            64 => {
                let pcvalue: i64 = i64::from(pc.clone()) + offset as i64;

                println!("{:0x}", pcvalue)
            },
            128 => {
                let pcvalue: i128 = i128::from(pc.clone()) + offset as i128;

                println!("{:0x}", pcvalue)
            },
            _ => return Err(exception::RvException::InstructionIllegal),
        }

        if branched {
            Ok(offset as i128)
        } else {
            Ok(4)
        }
    }

    fn store(&self, reg: &mut RvRegisters, bus: &mut Bus) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();
        let imm: i32 = self.get_s_imm();

        let result = match funct3 {
            0x0 => store::sb(reg, rs1, rs2, imm, bus), // store byte
            0x1 => store::sh(reg, rs1, rs2, imm, bus), // store half
            0x2 => store::sw(reg, rs1, rs2, imm, bus), // store word
            0x3 => store::sd(reg, rs1, rs2, imm, bus), // store double
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        match result {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    fn system(&self, reg: &mut RvRegisters, csr: &mut Csr) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let funct12: usize = self.get_funct12();

        match funct3 {
            0x0 => match rs1 {
                0x0 => system::ecall(reg, csr), // ecall
                0x1 => system::ebreak(reg, csr), // ebreak
                0x2 => system::xret(reg, rd, rs1, funct12, csr), // xret: sret, mret, mnret
                _ => return Some(exception::RvException::InstructionIllegal),
            },
            0x1 | 0x5 => system::csrrw(reg, rd, rs1, funct3, funct12, csr), // csrrw, csrrwi
            0x2 | 0x6 => system::csrrs(reg, rd, rs1, funct3, funct12, csr), // csrrs, csrrsi
            0x3 | 0x7 => system::csrrc(reg, rd, rs1, funct3, funct12, csr), // csrrc, csrrci
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        None
    }

    fn process_32(&self, hart: &mut Rv, bus: &mut Bus) -> Result<i128, exception::RvException> {
        let mut offset: i128 = 4;
        match self.get_opcode() {
            0x00 => {
                match self.load(&mut hart.reg, bus) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            //            0x01 => rc = self.load_fp(),
            //            0x02 => rc = self.custom_0(),
            0x03 => {
                match self.mem() {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            0x04 => {
                match self.opimm(&mut hart.reg) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            0x05 =>  {
                match self.auipc(&mut hart.reg, &hart.pc) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            0x06 => {
                match self.opimm32(&mut hart.reg) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            //
            0x08 => {
                match self.store(&mut hart.reg, bus) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            //            0x09 => rc = self.store_fp(),
            //            0x0a => rc = self.custom_1(),
            //            0x0b => rc = self.amo(),
            0x0c =>  {
                match self.op(&mut hart.reg) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            0x0d =>  {
                match self.lui(&mut hart.reg) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            0x0e =>  {
                match self.op32(&mut hart.reg) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            //
            //            0x10 => rc = self.madd(),
            //            0x11 => rc = self.msub(),
            //            0x12 => rc = self.nmsub(),
            //            0x13 => rc = self.nmadd(),
            //            0x14 => rc = self.op_fp(),
            //            0x16 => rc = self.custom_2(),
            //
            0x18 =>  {
                match self.branch(&mut hart.reg, &hart.pc) {
                    Ok(o) => {
                        match hart.reg.width() {
                            32 | 64 | 128 => offset = o,
                            _ => return Err(exception::RvException::InstructionIllegal),
                        }
                    },
                    Err(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            0x19 =>  {
                match self.jalr(&mut hart.reg, &hart.pc) {
                    Ok(o) => {
                        match hart.reg.width() {
                            32 | 64 | 128 => offset = o,
                            _ => return Err(exception::RvException::InstructionIllegal),
                        }
                    },
                    Err(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            0x1b =>  {
                match self.jal(&mut hart.reg, &hart.pc) {
                    Ok(o) => {
                        match hart.reg.width() {
                            32 | 64 | 128 => offset = o,
                            _ => return Err(exception::RvException::InstructionIllegal),
                        }
                    },
                    Err(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            0x1c =>  {
                match self.system(&mut hart.reg, &mut hart.csr) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    },
                }
            },
            //            0x1e => rc = self.custom_3(),
            //
            _ => {
                println!("<invalid>");
                return Err(exception::RvException::InstructionIllegal);
            },
        }

        Ok(offset)
    }

    pub fn process(&self, hart: &mut Rv, bus: &mut Bus) -> Result<i128, exception::RvException> {
        print!("asm: \t{}:\t{:08x}\t\t", hart.pc, self.get_raw());

        match self {
            Instr::Instr32(_) => match self.process_32(hart, bus) {
                Ok(offset) => Ok(offset),
                Err(e) => Err(e),
            },
            _ => Err(exception::RvException::InstructionIllegal),
        }
    }

    fn get_opcode(&self) -> usize {
        match self {
            Instr::Invalid => unreachable!(),
            Instr::InstrC0(i) => ((i & 0x7f) >> 2) as usize,
            Instr::InstrC1(i) => ((i & 0x7f) >> 2) as usize,
            Instr::InstrC2(i) => ((i & 0x7f) >> 2) as usize,
            Instr::Instr32(i) => ((i & 0x7f) >> 2) as usize,
        }
    }

    fn get_rd(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 7) & 0x1f) as usize,
            Instr::InstrC1(i) => ((i >> 7) & 0x1f) as usize,
            Instr::InstrC2(i) => ((i >> 7) & 0x1f) as usize,
            Instr::Instr32(i) => ((i >> 7) & 0x1f) as usize,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_rs1(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 7) & 0x1f) as usize,
            Instr::InstrC1(i) => ((i >> 7) & 0x1f) as usize,
            Instr::InstrC2(i) => ((i >> 7) & 0x1f) as usize,
            Instr::Instr32(i) => ((i >> 15) & 0x1f) as usize,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_rs2(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 2) & 0x1f) as usize,
            Instr::InstrC1(i) => ((i >> 2) & 0x1f) as usize,
            Instr::InstrC2(i) => ((i >> 2) & 0x1f) as usize,
            Instr::Instr32(i) => ((i >> 20) & 0x1f) as usize,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_rd_prime(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 2) & 0x03) as usize,
            Instr::InstrC1(i) => ((i >> 2) & 0x03) as usize,
            Instr::InstrC2(i) => ((i >> 2) & 0x03) as usize,
            Instr::Instr32(_) => todo!(),
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_rs1_prime(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 7) & 0x03) as usize,
            Instr::InstrC1(i) => ((i >> 7) & 0x03) as usize,
            Instr::InstrC2(i) => ((i >> 7) & 0x03) as usize,
            Instr::Instr32(_) => todo!(),
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_rs2_prime(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 2) & 0x03) as usize,
            Instr::InstrC1(i) => ((i >> 2) & 0x03) as usize,
            Instr::InstrC2(i) => ((i >> 2) & 0x03) as usize,
            Instr::Instr32(_) => todo!(),
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_funct3(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 13) & 0x02) as usize,
            Instr::InstrC1(i) => ((i >> 13) & 0x02) as usize,
            Instr::InstrC2(i) => ((i >> 13) & 0x02) as usize,
            Instr::Instr32(i) => ((i >> 12) & 0x07) as usize,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_funct4(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 11) & 0x0f) as usize,
            Instr::InstrC1(i) => ((i >> 11) & 0x0f) as usize,
            Instr::InstrC2(i) => ((i >> 11) & 0x0f) as usize,
            Instr::Instr32(_) => todo!(),
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_funct5(&self) -> usize {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(i) => ((i >> 27) & 0x1f) as usize,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_funct6(&self) -> usize {
        match self {
            Instr::InstrC0(i) => ((i >> 10) & 0x3f) as usize,
            Instr::InstrC1(i) => ((i >> 10) & 0x3f) as usize,
            Instr::InstrC2(i) => ((i >> 10) & 0x3f) as usize,
            Instr::Instr32(_) => todo!(),
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_funct7(&self) -> usize {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(i) => ((i >> 25) & 0x2f) as usize,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_funct12(&self) -> usize {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(i) => (*i as usize >> 20) & 0xfff,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_shamt(&self) -> usize {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(i) => ((i >> 20) & 0x3f) as usize,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_imm(&self, last: usize, first: usize) -> u32 {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(i) => ((*i as u64 >> first) & (((1u64 << (last + 1)) - 1) >> first)).try_into().unwrap(),
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_i_imm(&self) -> i32 {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(_) => ((self.get_imm(31, 20) << 20) as i32) >> 20,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_s_imm(&self) -> i32 {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(_) => ((((self.get_imm(31, 25) << 5)
            | self.get_imm(11, 7)) << 20) as i32) >> 20,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_b_imm(&self) -> i32 {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(_) => ((((self.get_imm(31, 31) << 12)
            | (self.get_imm(7, 7) << 11)
            | (self.get_imm(30, 25) << 5)
            | (self.get_imm(11, 8) << 1)) << 19) as i32) >> 19,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_u_imm(&self) -> i32 {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(_) => (self.get_imm(31, 12) as i32) << 12,
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_j_imm(&self) -> i32 {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(_) => ((((self.get_imm(31, 31) << 20)
            | (self.get_imm(19, 12) << 12)
            | (self.get_imm(20, 20) << 11)
            | (self.get_imm(30, 21) << 1)) << 12) as i32 ) >> 12,
            Instr::Invalid => unreachable!(),
        }
    }
}

