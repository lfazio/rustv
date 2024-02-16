mod amo;
mod auipc;
mod branch;
mod load;
mod lui;
mod op;
mod opimm;
mod store;
mod system;

use super::atomic::AtomicCtx;
use super::csr::Csr;
use super::exception;
use super::ext::RvExtensions;
use super::hart::Rv;
use super::registers::RvRegisters;
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
            }
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

    fn load(&self, x: &mut RvRegisters, bus: &mut Bus) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let imm: i32 = self.get_i_imm();

        let result = match funct3 {
            0x0 => load::lb(x, rd, rs1, imm, bus),  // load byte
            0x1 => load::lh(x, rd, rs1, imm, bus),  // load half
            0x2 => load::lw(x, rd, rs1, imm, bus),  // load word
            0x3 => load::ld(x, rd, rs1, imm, bus),  // load double
            0x4 => load::lbu(x, rd, rs1, imm, bus), // load byte unsigned
            0x5 => load::lhu(x, rd, rs1, imm, bus), // load half unsigned
            0x6 => load::lwu(x, rd, rs1, imm, bus), // load word unsigned
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        match result {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    fn op(&self, ext: &RvExtensions, x: &mut RvRegisters) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let funct7: usize = self.get_funct7();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();

        match self.get_funct7() {
            0x01 => match funct3 {
                0x0 => {
                    if ext.m || ext.zmmul {
                        op::mul(x, rd, rs1, rs2);
                    } else {
                        return Some(exception::RvException::InstructionIllegal);
                    }
                }
                0x1 => {
                    if ext.m || ext.zmmul {
                        op::mulh(x, rd, rs1, rs2);
                    } else {
                        return Some(exception::RvException::InstructionIllegal);
                    }
                }
                0x2 => {
                    if ext.m || ext.zmmul {
                        op::mulhsu(x, rd, rs1, rs2);
                    } else {
                        return Some(exception::RvException::InstructionIllegal);
                    }
                }
                0x3 => {
                    if ext.m || ext.zmmul {
                        op::mulhu(x, rd, rs1, rs2);
                    } else {
                        return Some(exception::RvException::InstructionIllegal);
                    }
                }
                0x4 => {
                    if ext.m {
                        op::div(x, rd, rs1, rs2);
                    } else {
                        return Some(exception::RvException::InstructionIllegal);
                    }
                }
                0x5 => {
                    if ext.m {
                        op::divu(x, rd, rs1, rs2);
                    } else {
                        return Some(exception::RvException::InstructionIllegal);
                    }
                }
                0x6 => {
                    if ext.m {
                        op::rem(x, rd, rs1, rs2);
                    } else {
                        return Some(exception::RvException::InstructionIllegal);
                    }
                }
                0x7 => {
                    if ext.m {
                        op::remu(x, rd, rs1, rs2);
                    } else {
                        return Some(exception::RvException::InstructionIllegal);
                    }
                }
                _ => return Some(exception::RvException::InstructionIllegal),
            },
            _ => match funct3 {
                0x0 => match self.get_funct7() {
                    0x00 => op::add(x, rd, rs1, rs2), // add
                    0x20 => op::sub(x, rd, rs1, rs2), // sub
                    _ => return Some(exception::RvException::InstructionIllegal),
                },
                0x1 => op::sll(x, rd, rs1, rs2),  // sll
                0x2 => op::slt(x, rd, rs1, rs2),  // slt
                0x3 => op::sltu(x, rd, rs1, rs2), // sltu
                0x4 => op::xor(x, rd, rs1, rs2),  // xor
                0x5 => {
                    if (funct7 & 0x20) == 0x20 {
                        op::sra(x, rd, rs1, rs2); // sra
                    } else {
                        op::srl(x, rd, rs1, rs2); // srl
                    }
                }
                0x6 => op::or(x, rd, rs1, rs2),  // or
                0x7 => op::and(x, rd, rs1, rs2), // and
                _ => return Some(exception::RvException::InstructionIllegal),
            },
        };

        None
    }

    fn op32(&self, ext: &RvExtensions, x: &mut RvRegisters) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();

        if x.len() == 32 {
            return Some(exception::RvException::InstructionIllegal);
        }

        match self.get_funct7() {
            0x01 => match funct3 {
                0x0 => if ext.m || ext.zmmul {
                    op::mulw(x, rd, rs1, rs2);
                } else {
                    return Some(exception::RvException::InstructionIllegal);
                },
                0x4 => if ext.m {
                    op::divw(x, rd, rs1, rs2);
                } else {
                    return Some(exception::RvException::InstructionIllegal);
                },
                0x5 => if ext.m {
                    op::divuw(x, rd, rs1, rs2);
                } else {
                    return Some(exception::RvException::InstructionIllegal);
                },
                0x6 => if ext.m {
                    op::remw(x, rd, rs1, rs2);
                } else {
                    return Some(exception::RvException::InstructionIllegal);
                },
                0x7 => if ext.m {
                    op::remuw(x, rd, rs1, rs2);
                } else {
                    return Some(exception::RvException::InstructionIllegal);
                },
                _ => return Some(exception::RvException::InstructionIllegal),
            },
            _ => match funct3 {
                0x0 => match self.get_funct7() {
                    0x00 => op::addw(x, rd, rs1, rs2), // addw
                    0x20 => op::subw(x, rd, rs1, rs2), // subw
                    _ => return Some(exception::RvException::InstructionIllegal),
                },
                0x1 => op::sllw(x, rd, rs1, rs2), // sllw
                0x5 => match self.get_funct7() {
                    0x00 => op::srlw(x, rd, rs1, rs2), // srlw
                    0x20 => op::sraw(x, rd, rs1, rs2), // sraw
                    _ => return Some(exception::RvException::InstructionIllegal),
                },
                _ => return Some(exception::RvException::InstructionIllegal),
            },
        };

        None
    }

    fn mem(&self, zifencei: bool) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let fm: usize = self.get_imm(31, 28) as usize >> 28;

        match funct3 {
            0x0 => match fm {
                0x0 => println!("fence"),
                0x8 => println!("fence.tso"),
                _ => return Some(exception::RvException::InstructionIllegal),
            },
            0x1 => {
                if zifencei {
                    println!("fence.i");
                } else {
                    return Some(exception::RvException::InstructionIllegal);
                }
            }
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        None
    }

    fn opimm(&self, x: &mut RvRegisters) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let funct7: usize = self.get_funct7();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let imm: i32 = self.get_i_imm();
        let shamt: usize = self.get_shamt();

        match funct3 {
            0x0 => opimm::addi(x, rd, rs1, imm),   // addi
            0x1 => opimm::slli(x, rd, rs1, shamt), // slli
            0x2 => opimm::slti(x, rd, rs1, imm),   // slti
            0x3 => opimm::sltiu(x, rd, rs1, imm),  // sltiu
            0x4 => opimm::xori(x, rd, rs1, imm),   // xori
            0x5 => {
                if funct7 & 0x20 == 0x20 {
                    opimm::srai(x, rd, rs1, shamt); // srai
                } else {
                    opimm::srli(x, rd, rs1, shamt); // srli
                }
            }
            0x6 => opimm::ori(x, rd, rs1, imm),  // ori
            0x7 => opimm::andi(x, rd, rs1, imm), // andi
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        None
    }

    fn opimm32(&self, x: &mut RvRegisters) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let funct7: usize = self.get_funct7();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();
        let imm: i32 = self.get_i_imm();

        if x.len() == 32 {
            return Some(exception::RvException::InstructionIllegal);
        }

        match funct3 {
            0x0 => opimm::addiw(x, rd, rs1, imm), // addiw
            0x1 => opimm::slliw(x, rd, rs1, rs2), // slliw
            0x5 => {
                if funct7 & 0x20 == 0x20 {
                    opimm::sraiw(x, rd, rs1, rs2); // sraiw
                } else {
                    opimm::srliw(x, rd, rs1, rs2); // srliw
                }
            }
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        None
    }

    fn auipc(&self, x: &mut RvRegisters, pc: &Uint) -> Option<exception::RvException> {
        let rd: usize = self.get_rd();
        let imm: i32 = self.get_u_imm();

        auipc::auipc(x, rd, pc, imm);

        None
    }

    fn lui(&self, x: &mut RvRegisters) -> Option<exception::RvException> {
        let rd: usize = self.get_rd();
        let imm: i32 = self.get_u_imm();

        lui::lui(x, rd, imm);

        None
    }

    fn jalr(&self, x: &mut RvRegisters, pc: &Uint) -> Result<i128, exception::RvException> {
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let imm: i32 = self.get_i_imm();
        let v: u128;
        let offset: i128;

        match x.len() {
            32 => {
                let base: i32 = i32::from(x.get(rs1));
                let value: i32 = i32::from(pc.clone());

                v = (base as u128 + 4) & 0xffffffff;
                x.set(rd, &Uint::from(value + 4));
                offset = (base - value) as i128 + imm as i128;
            }
            64 => {
                let base: i64 = i64::from(x.get(rs1));
                let value: i64 = i64::from(pc.clone());

                v = (base as u128 + 4) & 0xffffffffffffffff;
                x.set(rd, &Uint::from(value + 4));
                offset = (base - value) as i128 + imm as i128;
            }
            128 => {
                let base: i128 = i128::from(x.get(rs1));
                let value: i128 = i128::from(pc.clone());

                v = base as u128 + 4;
                x.set(rd, &Uint::from(value + 4));
                offset = base - value + imm as i128;
            }
            _ => unreachable!(),
        }

        println!("jalr\t{},{:0x}", x.name(rd), v);

        Ok(offset)
    }

    fn jal(&self, x: &mut RvRegisters, pc: &Uint) -> Result<i128, exception::RvException> {
        let rd: usize = self.get_rd();
        let imm: i32 = self.get_j_imm();
        let v: u128;

        match x.len() {
            32 => {
                let value: u32 = u32::from(pc.clone()) + 4;

                v = value as u128;
                x.set(rd, &Uint::from(value));
            }
            64 => {
                let value: u64 = u64::from(pc.clone()) + 4;

                v = value as u128;
                x.set(rd, &Uint::from(value));
            }
            128 => {
                let value: u128 = u128::from(pc.clone()) + 4;

                v = value;
                x.set(rd, &Uint::from(value));
            }
            _ => unreachable!(),
        };

        if rd == 0 {
            println!("j\t{:0x}", v);
        } else {
            println!("jal\t{},{:0x}", x.name(rd), v);
        }

        Ok(imm as i128)
    }

    fn branch(&self, x: &mut RvRegisters, pc: &Uint) -> Result<i128, exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();
        let offset: i32 = self.get_b_imm();
        let branched: bool = match funct3 {
            0x0 => branch::beq(x, rs1, rs2).unwrap(),  // beq
            0x1 => branch::bne(x, rs1, rs2).unwrap(),  // bne
            0x4 => branch::blt(x, rs1, rs2).unwrap(),  // blt
            0x5 => branch::bge(x, rs1, rs2).unwrap(),  // bge
            0x6 => branch::bltu(x, rs1, rs2).unwrap(), // bltu
            0x7 => branch::bgeu(x, rs1, rs2).unwrap(), // bgeu
            _ => return Err(exception::RvException::InstructionIllegal),
        };

        match x.len() {
            32 => {
                let pcvalue: i32 = i32::from(pc.clone()) + offset;

                println!("{:0x}\t# {} {}", pcvalue, x.get(rs1), x.get(rs2))
            }
            64 => {
                let pcvalue: i64 = i64::from(pc.clone()) + offset as i64;

                println!("{:0x}\t# {} {}", pcvalue, x.get(rs1), x.get(rs2))
            }
            128 => {
                let pcvalue: i128 = i128::from(pc.clone()) + offset as i128;

                println!("{:0x}\t# {} {}", pcvalue, x.get(rs1), x.get(rs2))
            }
            _ => return Err(exception::RvException::InstructionIllegal),
        }

        if branched {
            Ok(offset as i128)
        } else {
            Ok(4)
        }
    }

    fn store(&self, x: &mut RvRegisters, bus: &mut Bus) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();
        let imm: i32 = self.get_s_imm();

        let result = match funct3 {
            0x0 => store::sb(x, rs1, rs2, imm, bus), // store byte
            0x1 => store::sh(x, rs1, rs2, imm, bus), // store half
            0x2 => store::sw(x, rs1, rs2, imm, bus), // store word
            0x3 => store::sd(x, rs1, rs2, imm, bus), // store double
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        match result {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    fn amo(&self, x: &mut RvRegisters, atomic_ctx: &mut AtomicCtx, extensions: &RvExtensions, bus: &mut Bus) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let rs2: usize = self.get_rs2();
        let funct7: usize = self.get_funct7();
        let result: Option<exception::RvException>;
        let rl: bool = (funct7 & 0x1) != 0;
        let aq: bool = (funct7 & 0x2) != 0;

        if funct3 == 0x3 && x.len() < 64 {
            return Some(exception::RvException::InstructionIllegal);
        }

        match funct7 >> 2 {
            0x00 => if extensions.zamo {
                result = amo::add(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x01 => if extensions.zamo {
                result = amo::swap(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x02 => if extensions.zalrsc {
                result = amo::lr(atomic_ctx, aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x03 => if extensions.zalrsc {
                result = amo::sc(atomic_ctx, aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x04 =>  if extensions.zamo {
                result = amo::xor(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x05 =>  if extensions.zacas {
                if funct3 == 0x4 && x.len() < 64 {
                    return Some(exception::RvException::InstructionIllegal);
                }
                result = amo::cas(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x08 =>  if extensions.zamo {
                result = amo::or(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x0c =>  if extensions.zamo {
                result = amo::and(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x10 =>  if extensions.zamo {
                result = amo::min(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x14 =>  if extensions.zamo {
                result = amo::max(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x18 =>  if extensions.zamo {
                result = amo::minu(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            0x1c =>  if extensions.zamo {
                result = amo::maxu(aq, rl, funct3, x, rd, rs1, rs2, bus);
            } else {
                return Some(exception::RvException::InstructionIllegal);
            },
            _ => return Some(exception::RvException::InstructionIllegal),
        };

        result
    }

    fn system(
        &self,
        x: &mut RvRegisters,
        csr: &mut Option<Csr>,
    ) -> Option<exception::RvException> {
        let funct3: usize = self.get_funct3();
        let rd: usize = self.get_rd();
        let rs1: usize = self.get_rs1();
        let funct12: usize = self.get_funct12();

        match funct3 {
            0x0 => match rs1 {
                0x0 => system::ecall(x),  // ecall
                0x1 => system::ebreak(x), // ebreak
                0x2 => {
                    if let Some(c) = csr {
                        system::xret(x, rd, rs1, funct12, c) // xret: sret, mret, mnret
                    } else {
                        Some(exception::RvException::InstructionIllegal)
                    }
                }
                _ => Some(exception::RvException::InstructionIllegal),
            },
            0x1 | 0x5 => {
                if let Some(c) = csr {
                    system::csrrw(x, rd, rs1, funct3, funct12, c) // csrrw, csrrwi
                } else {
                    Some(exception::RvException::InstructionIllegal)
                }
            }
            0x2 | 0x6 => {
                if let Some(c) = csr {
                    system::csrrs(x, rd, rs1, funct3, funct12, c) // csrrs, csrrsi
                } else {
                    Some(exception::RvException::InstructionIllegal)
                }
            }
            0x3 | 0x7 => {
                if let Some(c) = csr {
                    system::csrrc(x, rd, rs1, funct3, funct12, c) // csrrc, csrrci
                } else {
                    Some(exception::RvException::InstructionIllegal)
                }
            }
            _ => Some(exception::RvException::InstructionIllegal),
        }
    }

    fn process_32(&self, hart: &mut Rv, bus: &mut Bus) -> Result<i128, exception::RvException> {
        let mut offset: i128 = 4;
        match self.get_opcode() {
            0x00 => match self.load(&mut hart.x, bus) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            //            0x01 => rc = self.load_fp(),
            //            0x02 => rc = self.custom_0(),
            0x03 => match self.mem(hart.extensions.zifencei) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x04 => match self.opimm(&mut hart.x) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x05 => match self.auipc(&mut hart.x, &hart.pc) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x06 => match self.opimm32(&mut hart.x) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            //
            0x08 => match self.store(&mut hart.x, bus) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x09 => {
                if hart.f.is_none() {
                    return Err(exception::RvException::InstructionIllegal);
                }
                match self.store_fp(&mut hart.x, hart.f.as_mut().unwrap(), &hart.extensions, bus) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    }
                }
            },
            //            0x0a => rc = self.custom_1(),
            0x0b => {
                if hart.atomic_ctx.is_none() {
                    return Err(exception::RvException::InstructionIllegal);
                }
                match self.amo(&mut hart.x, hart.atomic_ctx.as_mut().unwrap(), &hart.extensions, bus) {
                    None => (),
                    Some(e) => {
                        println!("<error>");
                        return Err(e);
                    }
                }
            },
            0x0c => match self.op(&hart.extensions, &mut hart.x) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x0d => match self.lui(&mut hart.x) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x0e => match self.op32(&hart.extensions, &mut hart.x) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
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
            0x18 => match self.branch(&mut hart.x, &hart.pc) {
                Ok(o) => match hart.x.len() {
                    32 | 64 | 128 => offset = o,
                    _ => return Err(exception::RvException::InstructionIllegal),
                },
                Err(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x19 => match self.jalr(&mut hart.x, &hart.pc) {
                Ok(o) => match hart.x.len() {
                    32 | 64 | 128 => offset = o,
                    _ => return Err(exception::RvException::InstructionIllegal),
                },
                Err(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x1b => match self.jal(&mut hart.x, &hart.pc) {
                Ok(o) => match hart.x.len() {
                    32 | 64 | 128 => offset = o,
                    _ => return Err(exception::RvException::InstructionIllegal),
                },
                Err(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            0x1c => match self.system(&mut hart.x, &mut hart.csr) {
                None => (),
                Some(e) => {
                    println!("<error>");
                    return Err(e);
                }
            },
            //            0x1e => rc = self.custom_3(),
            //
            _ => {
                println!("<invalid>");
                return Err(exception::RvException::InstructionIllegal);
            }
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
            Instr::Instr32(i) => ((i >> 25) & 0x7f) as usize,
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
            Instr::Instr32(i) => ((*i as u64 >> first) & (((1u64 << (last + 1)) - 1) >> first))
                .try_into()
                .unwrap(),
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
            Instr::Instr32(_) => {
                ((((self.get_imm(31, 25) << 5) | self.get_imm(11, 7)) << 20) as i32) >> 20
            }
            Instr::Invalid => unreachable!(),
        }
    }

    fn get_b_imm(&self) -> i32 {
        match self {
            Instr::InstrC0(_) => todo!(),
            Instr::InstrC1(_) => todo!(),
            Instr::InstrC2(_) => todo!(),
            Instr::Instr32(_) => {
                ((((self.get_imm(31, 31) << 12)
                    | (self.get_imm(7, 7) << 11)
                    | (self.get_imm(30, 25) << 5)
                    | (self.get_imm(11, 8) << 1))
                    << 19) as i32)
                    >> 19
            }
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
            Instr::Instr32(_) => {
                ((((self.get_imm(31, 31) << 20)
                    | (self.get_imm(19, 12) << 12)
                    | (self.get_imm(20, 20) << 11)
                    | (self.get_imm(30, 21) << 1))
                    << 12) as i32)
                    >> 12
            }
            Instr::Invalid => unreachable!(),
        }
    }
}
