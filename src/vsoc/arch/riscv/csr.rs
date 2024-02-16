use std::fmt;

// Unpriviledge Floating-Point CSRs
pub const FFLAGS: usize = 0x001;
pub const FRM: usize = 0x002;
pub const FCSR: usize = 0x003;

// Unpriviledge Counter/Timers CSRs
pub const CYCLE: usize = 0xc00;
pub const TIME: usize = 0xc01;
pub const INSTRET: usize = 0xc02;
pub const HPMCOUNTER3: usize = 0xc03; //..0xc1f
pub const CYCLEH: usize = 0xc80;
pub const TIMEH: usize = 0xc81;
pub const INSTRETH: usize = 0xc82;
pub const HPMCOUNTER3H: usize = 0xc83; //..0xc9f

// Supervisor Trap Setup
pub const SSTATUS: usize = 0x100;
pub const SIE: usize = 0x104;
pub const STVEC: usize = 0x105;
pub const SCOUNTEREN: usize = 0x106;
// Supervisor Configuration
pub const SENVCFG: usize = 0x10a;
// Supervisor Trap Handling
pub const SSCRATCH: usize = 0x140;
pub const SEPC: usize = 0x141;
pub const SCAUSE: usize = 0x142;
pub const STVAL: usize = 0x143;
pub const SIP: usize = 0x144;
// Supervisor Protection and Translation
pub const SATP: usize = 0x180;
// Debug/Trace Registers
pub const SCONTEXT: usize = 0x5a8;

// Hypervisor Trap Setup
pub const HSTATUS: usize = 0x600;
pub const HEDELEG: usize = 0x602;
pub const HIDELEG: usize = 0x603;
pub const HIE: usize = 0x604;
pub const HCOUNTEREN: usize = 0x606;
pub const HGEIE: usize = 0x607;
// Hypervisor Trap Handling
pub const HTVAL: usize = 0x643;
pub const HIP: usize = 0x644;
pub const HVIP: usize = 0x645;
pub const HTINST: usize = 0x64a;
pub const HGEIP: usize = 0xe12;
// Hypervisor Configuration
pub const HENVCFG: usize = 0x60a;
pub const HENVCFGH: usize = 0x61a;
// Hypervisor Protection and Translation
pub const HGATP: usize = 0x680;
// Debug/Trace Registers
pub const HCONTEXT: usize = 0x6a8;
// Hypervisor Counter/Timers Virtualisation Registers
pub const HTIMEDELTA: usize = 0x605;
pub const HTIMEDELTAH: usize = 0x615;

// Virtual Supervisor Registers
pub const VSSTATUS: usize = 0x200;
pub const VSIE: usize = 0x204;
pub const VSTVEC: usize = 0x205;
pub const VSSCRATCH: usize = 0x240;
pub const VSEPC: usize = 0x241;
pub const VSCAUSE: usize = 0x242;
pub const VSTVAL: usize = 0x243;
pub const VSIP: usize = 0x244;
pub const VSATP: usize = 0x280;

// Machine Information Registers
pub const MVENDORID: usize = 0xf11;
pub const MARCHID: usize = 0xf12;
pub const MIMPID: usize = 0xf13;
pub const MHARTID: usize = 0xf14;
pub const MCONFIGPTR: usize = 0xf15;
// Machine Trap Setup
pub const MSTATUS: usize = 0x300;
pub const MISA: usize = 0x301;
pub const MEDELEG: usize = 0x302;
pub const MIDELEG: usize = 0x303;
pub const MIE: usize = 0x304;
pub const MTVEC: usize = 0x305;
pub const MCOUNTEREN: usize = 0x306;
pub const MSTATUSH: usize = 0x310;
// Machine Trap Handling
pub const MSCRATCH: usize = 0x340;
pub const MEPC: usize = 0x341;
pub const MCAUSE: usize = 0x342;
pub const MTVAL: usize = 0x343;
pub const MIP: usize = 0x344;
pub const MTINST: usize = 0x34a;
pub const MTVAL2: usize = 0x34b;
// Machine Configuration
pub const MENVCFG: usize = 0x30a;
pub const MENVCFGH: usize = 0x31a;
pub const MSECCFG: usize = 0x747;
pub const MSECCFGH: usize = 0x757;
// Machine Memory Protection
pub const PMPCFG0: usize = 0x3a0; //..0x3af
pub const PMPADDR0: usize = 0x3b0; //..0x3ef
                                   // Machine Non-Maskable Interrupt Handling
pub const MNSCRATCH: usize = 0x740;
pub const MNEPC: usize = 0x741;
pub const MNCAUSE: usize = 0x742;
pub const MNSTATUS: usize = 0x744;
// Machine Counter/Timers
pub const MCYCLE: usize = 0xb00;
pub const MINSTRET: usize = 0xb02;
pub const MHPMCOUNTER3: usize = 0xb03; //..0xb1f
pub const MCYCLEH: usize = 0xb80;
pub const MINSTRETH: usize = 0xb82;
pub const MHPMCOUNTER3H: usize = 0xb83; //..0xb9f
                                        // Machine Counter Setup
pub const MCOUNTINHIBIT: usize = 0x320;
pub const MHPMEVENT3: usize = 0x323; //..0x33f
                                     // Debug/Trace Registers (shared with Debug Mode)
pub const TSELECT: usize = 0x7a0;
pub const TDATA1: usize = 0x7a1;
pub const TDATA2: usize = 0x7a2;
pub const TDATA3: usize = 0x7a3;
pub const MCONTEXT: usize = 0x7a8;
// Debug Mode Registers
pub const DCSR: usize = 0x7b0;
pub const DPC: usize = 0x7b1;
pub const DSCRATCH0: usize = 0x7b2;
pub const DSCRATCH1: usize = 0x7b3;

use crate::vsoc::arch::{registers::ArchRegister, types::Uint};

use super::ext::RvExtensions;

#[derive(Debug, Default)]
pub struct Csr {
    xlen: usize,
    bank: Vec<ArchRegister>,
}

impl Csr {
    pub fn new(xlen: usize, extensions: &RvExtensions) -> Csr {
        let mut csr: Vec<ArchRegister> = Vec::<ArchRegister>::with_capacity(4096);

        println!("* Creating CSR registers");

        for i in 0..4096 {
            csr.push(ArchRegister::new(
                String::from("invalid"),
                i,
                Uint::zero(xlen),
            ));
        }

        println!("* Populating CSR registers");
        if extensions.f {
            csr[FFLAGS] = ArchRegister::new(String::from("fflags"), FFLAGS, Uint::zero(xlen));
            csr[FRM] = ArchRegister::new(String::from("frm"), FRM, Uint::zero(xlen));
            csr[FCSR] = ArchRegister::new(String::from("fcsr"), FCSR, Uint::zero(xlen));
        }

        if extensions.zicntr {
            csr[CYCLE] = ArchRegister::new(String::from("cycle"), CYCLE, Uint::zero(xlen));
            csr[TIME] = ArchRegister::new(String::from("time"), TIME, Uint::zero(xlen));
            csr[INSTRET] = ArchRegister::new(String::from("instret"), INSTRET, Uint::zero(xlen));
            if xlen == 32 {
                csr[CYCLEH] =
                    ArchRegister::new(String::from("cycleh"), CYCLEH, Uint::zero(xlen));
                csr[TIMEH] = ArchRegister::new(String::from("timeh"), TIMEH, Uint::zero(xlen));
                csr[INSTRETH] =
                    ArchRegister::new(String::from("instreth"), INSTRETH, Uint::zero(xlen));
            }
        }

        if extensions.zihpm {
            for i in 0..0x1c {
                csr[HPMCOUNTER3 + i] = ArchRegister::new(
                    format!("hpmcounter{}", i + 3),
                    HPMCOUNTER3 + i,
                    Uint::zero(xlen),
                );
                if xlen == 32 {
                    csr[HPMCOUNTER3H + i] = ArchRegister::new(
                        format!("hpmcounterh{}", i + 3),
                        HPMCOUNTER3H + i,
                        Uint::zero(xlen),
                    );
                }
            }
        }

        // Supervisor
        if extensions.s {
            println!("* Populating CSR registers for supervisor mode");
            csr[SSTATUS] = ArchRegister::new(String::from("sstatus"), SSTATUS, Uint::zero(xlen));
            csr[SIE] = ArchRegister::new(String::from("sie"), SIE, Uint::zero(xlen));
            csr[STVEC] = ArchRegister::new(String::from("stvec"), STVEC, Uint::zero(xlen));
            csr[SCOUNTEREN] =
                ArchRegister::new(String::from("scounteren"), SCOUNTEREN, Uint::zero(xlen));
            csr[SENVCFG] = ArchRegister::new(String::from("senvcfg"), SENVCFG, Uint::zero(xlen));
            csr[SSCRATCH] =
                ArchRegister::new(String::from("sscratch"), SSCRATCH, Uint::zero(xlen));
            csr[SEPC] = ArchRegister::new(String::from("sepc"), SEPC, Uint::zero(xlen));
            csr[SCAUSE] = ArchRegister::new(String::from("scause"), SCAUSE, Uint::zero(xlen));
            csr[STVAL] = ArchRegister::new(String::from("stval"), STVAL, Uint::zero(xlen));
            csr[SIP] = ArchRegister::new(String::from("sip"), SIP, Uint::zero(xlen));
            csr[SATP] = ArchRegister::new(String::from("satp"), SATP, Uint::zero(xlen));
            csr[SCONTEXT] =
                ArchRegister::new(String::from("scontext"), SCONTEXT, Uint::zero(xlen));
        }

        // Hypervisor
        if extensions.h {
            println!("* Populating CSR registers for hypervisor mode");
            csr[HSTATUS] = ArchRegister::new(String::from("hstatus"), HSTATUS, Uint::zero(xlen));
            csr[HEDELEG] = ArchRegister::new(String::from("hedeleg"), HEDELEG, Uint::zero(xlen));
            csr[HIDELEG] = ArchRegister::new(String::from("hideleg"), HIDELEG, Uint::zero(xlen));
            csr[HIE] = ArchRegister::new(String::from("hie"), HIE, Uint::zero(xlen));
            csr[HCOUNTEREN] =
                ArchRegister::new(String::from("hcounteren"), HCOUNTEREN, Uint::zero(xlen));
            csr[HGEIE] = ArchRegister::new(String::from("hgeie"), HGEIE, Uint::zero(xlen));
            csr[HTVAL] = ArchRegister::new(String::from("htval"), HTVAL, Uint::zero(xlen));
            csr[HIP] = ArchRegister::new(String::from("hip"), HIP, Uint::zero(xlen));
            csr[HVIP] = ArchRegister::new(String::from("hvip"), HVIP, Uint::zero(xlen));
            csr[HTINST] = ArchRegister::new(String::from("htinst"), HTINST, Uint::zero(xlen));
            csr[HGEIP] = ArchRegister::new(String::from("hgeip"), HGEIP, Uint::zero(xlen));
            csr[HENVCFG] = ArchRegister::new(String::from("henvcfg"), HENVCFG, Uint::zero(xlen));
            if xlen == 32 {
                csr[HENVCFGH] =
                    ArchRegister::new(String::from("henvcfgh"), HENVCFGH, Uint::zero(xlen));
            }
            csr[HGATP] = ArchRegister::new(String::from("hgatp"), HGATP, Uint::zero(xlen));
            csr[HCONTEXT] =
                ArchRegister::new(String::from("hcontext"), HCONTEXT, Uint::zero(xlen));
            csr[HTIMEDELTA] =
                ArchRegister::new(String::from("htimedelta"), HTIMEDELTA, Uint::zero(xlen));
            if xlen == 32 {
                csr[HTIMEDELTAH] = ArchRegister::new(
                    String::from("htimedeltah"),
                    HTIMEDELTAH,
                    Uint::zero(xlen),
                );
            }
            csr[VSSTATUS] =
                ArchRegister::new(String::from("vsstatus"), VSSTATUS, Uint::zero(xlen));
            csr[VSIE] = ArchRegister::new(String::from("vsie"), VSIE, Uint::zero(xlen));
            csr[VSTVEC] = ArchRegister::new(String::from("vstvec"), VSTVEC, Uint::zero(xlen));
            csr[VSSCRATCH] =
                ArchRegister::new(String::from("vsscratch"), VSSCRATCH, Uint::zero(xlen));
            csr[VSEPC] = ArchRegister::new(String::from("vsepc"), VSEPC, Uint::zero(xlen));
            csr[VSCAUSE] = ArchRegister::new(String::from("vscause"), VSCAUSE, Uint::zero(xlen));
            csr[VSTVAL] = ArchRegister::new(String::from("vstval"), VSTVAL, Uint::zero(xlen));
            csr[VSIP] = ArchRegister::new(String::from("vsip"), VSIP, Uint::zero(xlen));
            csr[VSATP] = ArchRegister::new(String::from("vsatp"), VSATP, Uint::zero(xlen));
        }

        // Machine
        println!("* Populating CSR registers for machine mode");
        csr[MVENDORID] =
            ArchRegister::new(String::from("mvendorid"), MVENDORID, Uint::zero(xlen));
        csr[MARCHID] = ArchRegister::new(String::from("marchid"), MARCHID, Uint::zero(xlen));
        csr[MIMPID] = ArchRegister::new(String::from("mimpid"), MIMPID, Uint::zero(xlen));
        csr[MHARTID] = ArchRegister::new(String::from("mhartid"), MHARTID, Uint::zero(xlen));
        csr[MCONFIGPTR] =
            ArchRegister::new(String::from("mconfigptr"), MCONFIGPTR, Uint::zero(xlen));
        csr[MSTATUS] = ArchRegister::new(String::from("mstatus"), MSTATUS, Uint::zero(xlen));
        if xlen == 32 {
            csr[MSTATUSH] =
                ArchRegister::new(String::from("mstatush"), MSTATUSH, Uint::zero(xlen));
        }
        csr[MISA] = ArchRegister::new(String::from("misa"), MISA, Uint::zero(xlen));
        csr[MEDELEG] = ArchRegister::new(String::from("medeleg"), MEDELEG, Uint::zero(xlen));
        csr[MIDELEG] = ArchRegister::new(String::from("mideleg"), MIDELEG, Uint::zero(xlen));
        csr[MIE] = ArchRegister::new(String::from("mie"), MIE, Uint::zero(xlen));
        csr[MTVEC] = ArchRegister::new(String::from("mtvec"), MTVEC, Uint::zero(xlen));
        csr[MCOUNTEREN] =
            ArchRegister::new(String::from("mcounteren"), MCOUNTEREN, Uint::zero(xlen));
        csr[MSCRATCH] = ArchRegister::new(String::from("mscratch"), MSCRATCH, Uint::zero(xlen));
        csr[MEPC] = ArchRegister::new(String::from("mepc"), MEPC, Uint::zero(xlen));
        csr[MCAUSE] = ArchRegister::new(String::from("mcause"), MCAUSE, Uint::zero(xlen));
        csr[MTVAL] = ArchRegister::new(String::from("mtval"), MTVAL, Uint::zero(xlen));
        csr[MIP] = ArchRegister::new(String::from("mip"), MIP, Uint::zero(xlen));
        csr[MTINST] = ArchRegister::new(String::from("mtinst"), MTINST, Uint::zero(xlen));
        csr[MTVAL2] = ArchRegister::new(String::from("mtval2"), MTVAL2, Uint::zero(xlen));
        csr[MENVCFG] = ArchRegister::new(String::from("menvcfg"), MENVCFG, Uint::zero(xlen));
        csr[MSECCFG] = ArchRegister::new(String::from("mseccfg"), MSECCFG, Uint::zero(xlen));
        if xlen == 32 {
            csr[MENVCFGH] =
                ArchRegister::new(String::from("menvcfgh"), MENVCFGH, Uint::zero(xlen));
            csr[MSECCFGH] =
                ArchRegister::new(String::from("mseccfgh"), MSECCFGH, Uint::zero(xlen));
        }
        for i in 0..0x0f {
            csr[PMPCFG0 + i] =
                ArchRegister::new(format!("pmpcfg{}", i), PMPCFG0 + i, Uint::zero(xlen));
            csr[PMPADDR0 + i] =
                ArchRegister::new(format!("pmpaddr{}", i), PMPADDR0 + i, Uint::zero(xlen));
        }
        csr[MNSCRATCH] =
            ArchRegister::new(String::from("mnscratch"), MNSCRATCH, Uint::zero(xlen));
        csr[MNEPC] = ArchRegister::new(String::from("mnepc"), MNEPC, Uint::zero(xlen));
        csr[MNCAUSE] = ArchRegister::new(String::from("mncause"), MNCAUSE, Uint::zero(xlen));
        csr[MNSTATUS] = ArchRegister::new(String::from("mnstatus"), MNSTATUS, Uint::zero(xlen));
        csr[MCYCLE] = ArchRegister::new(String::from("mcycle"), MCYCLE, Uint::zero(xlen));
        csr[MINSTRET] = ArchRegister::new(String::from("minstret"), MINSTRET, Uint::zero(xlen));
        if xlen == 32 {
            csr[MCYCLEH] = ArchRegister::new(String::from("mcycleh"), MCYCLE, Uint::zero(xlen));
            csr[MINSTRETH] =
                ArchRegister::new(String::from("minstreth"), MINSTRET, Uint::zero(xlen));
        }
        for i in 0..0x1c {
            csr[MHPMCOUNTER3 + i] = ArchRegister::new(
                format!("mhpmcounter{}", i + 3),
                MHPMCOUNTER3 + i,
                Uint::zero(xlen),
            );
            if xlen == 32 {
                csr[MHPMCOUNTER3H + i] = ArchRegister::new(
                    format!("mhpmcounterh{}", i + 3),
                    MHPMCOUNTER3H + i,
                    Uint::zero(xlen),
                );
            }
        }
        csr[MCOUNTINHIBIT] = ArchRegister::new(
            String::from("mcountinhibit"),
            MCOUNTINHIBIT,
            Uint::zero(xlen),
        );
        for i in 0..0x1c {
            csr[MHPMEVENT3 + i] = ArchRegister::new(
                format!("mhpmevent{}", i + 3),
                MHPMEVENT3 + i,
                Uint::zero(xlen),
            );
        }

        println!("* Populating CSR registers for debug mode");
        csr[TSELECT] = ArchRegister::new(String::from("tselect"), TSELECT, Uint::zero(xlen));
        csr[TDATA1] = ArchRegister::new(String::from("tdata1"), TDATA1, Uint::zero(xlen));
        csr[TDATA2] = ArchRegister::new(String::from("tdata2"), TDATA2, Uint::zero(xlen));
        csr[TDATA3] = ArchRegister::new(String::from("tdata3"), TDATA3, Uint::zero(xlen));
        csr[MCONTEXT] = ArchRegister::new(String::from("mcontext"), MCONTEXT, Uint::zero(xlen));
        csr[DCSR] = ArchRegister::new(String::from("dcsr"), DCSR, Uint::zero(xlen));
        csr[DPC] = ArchRegister::new(String::from("dpc"), DPC, Uint::zero(xlen));
        csr[DSCRATCH0] =
            ArchRegister::new(String::from("dscratch0"), DSCRATCH0, Uint::zero(xlen));
        csr[DSCRATCH1] =
            ArchRegister::new(String::from("dscratch1"), DSCRATCH1, Uint::zero(xlen));

        Csr { xlen, bank: csr }
    }

    pub fn name(&self, addr: usize) -> &str {
        self.bank[addr].name()
    }

    pub fn set(&mut self, addr: usize, value: &Uint) {
        if write_is_allowed(addr) {
            match addr {
                FFLAGS => {
                    let msk: Uint = Uint::from(0x1fu8).extend(self.xlen).clone();
                    let fcsr = self.bank[FCSR].get() & !msk.clone() | value.clone() & msk;
                    self.bank[FCSR].set(&fcsr);
                },
                FRM => {
                    let msk: Uint = Uint::from(0x3u8 << 5).extend(self.xlen).clone();
                    let shift: Uint = Uint::from(5u8).extend(self.xlen).clone();
                    let fcsr = self.bank[FCSR].get() & !msk.clone() | ((value.clone() & msk) << shift.clone());
                    self.bank[FCSR].set(&(fcsr >> shift));
                },
                _ => {
                    self.bank[addr].set(value);
                },
            }
        }
    }

    pub fn get(&self, addr: usize) -> Option<Uint> {
        if addr > self.bank.len() {
            return None;
        }

        if read_is_allowed(addr) {
            match addr {
                FFLAGS => {
                    let msk: Uint = Uint::from(0x1fu8).extend(self.xlen).clone();
                    return Some(self.bank[FCSR].get() & msk);
                },
                FRM => {
                    let msk: Uint = Uint::from(0x3u8 << 5).extend(self.xlen).clone();
                    let shift: Uint = Uint::from(5u8).extend(self.xlen).clone();
                    return Some((self.bank[FCSR].get() & msk) >> shift);
                },
                _ => {
                    return Some(self.bank[addr].get());
                },
            }
        }

        Some(Uint::zero(self.xlen))
    }
}

pub fn write_is_allowed(regidx: usize) -> bool {
    if regidx < 0xc00 {
        return true;
    }

    false
}

pub fn read_is_allowed(_regidx: usize) -> bool {
    true
}

impl fmt::Display for Csr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(csr ((xlen {})\n       (bank: {:?})))\n",
            self.xlen, self.bank
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::vsoc::arch::riscv::csr;

    #[test]
    fn test_write_allowed() {
        assert_eq!(csr::write_is_allowed(0x000), true);
        assert_eq!(csr::write_is_allowed(0xc00), false);
        assert_eq!(csr::write_is_allowed(0xfff), false);
    }

    #[test]
    fn test_read_allowed() {
        assert_eq!(csr::read_is_allowed(0x000), true);
        assert_eq!(csr::read_is_allowed(0xc00), true);
        assert_eq!(csr::read_is_allowed(0xfff), true);
    }
}
