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
    width: usize,
    bank: Vec<ArchRegister>,
}

impl Csr {
    pub fn new(width: usize, extensions: &RvExtensions) -> Csr {
        let mut csr: Vec<ArchRegister> = Vec::<ArchRegister>::with_capacity(4096);

        println!("* Creating CSR registers");

        for i in 0..4096 {
            csr.push(ArchRegister::new(
                width,
                String::from("invalid"),
                i,
                Uint::zero(width),
            ));
        }

        println!("* Populating CSR registers");
        if extensions.f {
            csr[FFLAGS] = ArchRegister::new(width, String::from("fflags"), FFLAGS, Uint::zero(width));
            csr[FRM] = ArchRegister::new(width, String::from("frm"), FRM, Uint::zero(width));
            csr[FCSR] = ArchRegister::new(width, String::from("fcsr"), FCSR, Uint::zero(width));
        }

        if extensions.zicntr {
            csr[CYCLE] = ArchRegister::new(width, String::from("cycle"), CYCLE, Uint::zero(width));
            csr[TIME] = ArchRegister::new(width, String::from("time"), TIME, Uint::zero(width));
            csr[INSTRET] = ArchRegister::new(width, String::from("instret"), INSTRET, Uint::zero(width));
            if width == 32 {
                csr[CYCLEH] =
                    ArchRegister::new(width, String::from("cycleh"), CYCLEH, Uint::zero(width));
                csr[TIMEH] = ArchRegister::new(width, String::from("timeh"), TIMEH, Uint::zero(width));
                csr[INSTRETH] =
                    ArchRegister::new(width, String::from("instreth"), INSTRETH, Uint::zero(width));
            }
        }

        if extensions.zihpm {
            for i in 0..0x1c {
                csr[HPMCOUNTER3 + i] = ArchRegister::new(
                    width,
                    format!("hpmcounter{}", i + 3),
                    HPMCOUNTER3 + i,
                    Uint::zero(width),
                );
                if width == 32 {
                    csr[HPMCOUNTER3H + i] = ArchRegister::new(
                        width,
                        format!("hpmcounterh{}", i + 3),
                        HPMCOUNTER3H + i,
                        Uint::zero(width),
                    );
                }
            }
        }

        // Supervisor
        if extensions.s {
            println!("* Populating CSR registers for supervisor mode");
            csr[SSTATUS] = ArchRegister::new(width, String::from("sstatus"), SSTATUS, Uint::zero(width));
            csr[SIE] = ArchRegister::new(width, String::from("sie"), SIE, Uint::zero(width));
            csr[STVEC] = ArchRegister::new(width, String::from("stvec"), STVEC, Uint::zero(width));
            csr[SCOUNTEREN] =
                ArchRegister::new(width, String::from("scounteren"), SCOUNTEREN, Uint::zero(width));
            csr[SENVCFG] = ArchRegister::new(width, String::from("senvcfg"), SENVCFG, Uint::zero(width));
            csr[SSCRATCH] =
                ArchRegister::new(width, String::from("sscratch"), SSCRATCH, Uint::zero(width));
            csr[SEPC] = ArchRegister::new(width, String::from("sepc"), SEPC, Uint::zero(width));
            csr[SCAUSE] = ArchRegister::new(width, String::from("scause"), SCAUSE, Uint::zero(width));
            csr[STVAL] = ArchRegister::new(width, String::from("stval"), STVAL, Uint::zero(width));
            csr[SIP] = ArchRegister::new(width, String::from("sip"), SIP, Uint::zero(width));
            csr[SATP] = ArchRegister::new(width, String::from("satp"), SATP, Uint::zero(width));
            csr[SCONTEXT] =
                ArchRegister::new(width, String::from("scontext"), SCONTEXT, Uint::zero(width));
        }

        // Hypervisor
        if extensions.h {
            println!("* Populating CSR registers for hypervisor mode");
            csr[HSTATUS] = ArchRegister::new(width, String::from("hstatus"), HSTATUS, Uint::zero(width));
            csr[HEDELEG] = ArchRegister::new(width, String::from("hedeleg"), HEDELEG, Uint::zero(width));
            csr[HIDELEG] = ArchRegister::new(width, String::from("hideleg"), HIDELEG, Uint::zero(width));
            csr[HIE] = ArchRegister::new(width, String::from("hie"), HIE, Uint::zero(width));
            csr[HCOUNTEREN] =
                ArchRegister::new(width, String::from("hcounteren"), HCOUNTEREN, Uint::zero(width));
            csr[HGEIE] = ArchRegister::new(width, String::from("hgeie"), HGEIE, Uint::zero(width));
            csr[HTVAL] = ArchRegister::new(width, String::from("htval"), HTVAL, Uint::zero(width));
            csr[HIP] = ArchRegister::new(width, String::from("hip"), HIP, Uint::zero(width));
            csr[HVIP] = ArchRegister::new(width, String::from("hvip"), HVIP, Uint::zero(width));
            csr[HTINST] = ArchRegister::new(width, String::from("htinst"), HTINST, Uint::zero(width));
            csr[HGEIP] = ArchRegister::new(width, String::from("hgeip"), HGEIP, Uint::zero(width));
            csr[HENVCFG] = ArchRegister::new(width, String::from("henvcfg"), HENVCFG, Uint::zero(width));
            if width == 32 {
                csr[HENVCFGH] =
                    ArchRegister::new(width, String::from("henvcfgh"), HENVCFGH, Uint::zero(width));
            }
            csr[HGATP] = ArchRegister::new(width, String::from("hgatp"), HGATP, Uint::zero(width));
            csr[HCONTEXT] =
                ArchRegister::new(width, String::from("hcontext"), HCONTEXT, Uint::zero(width));
            csr[HTIMEDELTA] =
                ArchRegister::new(width, String::from("htimedelta"), HTIMEDELTA, Uint::zero(width));
            if width == 32 {
                csr[HTIMEDELTAH] = ArchRegister::new(
                    width,
                    String::from("htimedeltah"),
                    HTIMEDELTAH,
                    Uint::zero(width),
                );
            }
            csr[VSSTATUS] =
                ArchRegister::new(width, String::from("vsstatus"), VSSTATUS, Uint::zero(width));
            csr[VSIE] = ArchRegister::new(width, String::from("vsie"), VSIE, Uint::zero(width));
            csr[VSTVEC] = ArchRegister::new(width, String::from("vstvec"), VSTVEC, Uint::zero(width));
            csr[VSSCRATCH] =
                ArchRegister::new(width, String::from("vsscratch"), VSSCRATCH, Uint::zero(width));
            csr[VSEPC] = ArchRegister::new(width, String::from("vsepc"), VSEPC, Uint::zero(width));
            csr[VSCAUSE] = ArchRegister::new(width, String::from("vscause"), VSCAUSE, Uint::zero(width));
            csr[VSTVAL] = ArchRegister::new(width, String::from("vstval"), VSTVAL, Uint::zero(width));
            csr[VSIP] = ArchRegister::new(width, String::from("vsip"), VSIP, Uint::zero(width));
            csr[VSATP] = ArchRegister::new(width, String::from("vsatp"), VSATP, Uint::zero(width));
        }

        // Machine
        println!("* Populating CSR registers for machine mode");
        csr[MVENDORID] =
            ArchRegister::new(width, String::from("mvendorid"), MVENDORID, Uint::zero(width));
        csr[MARCHID] = ArchRegister::new(width, String::from("marchid"), MARCHID, Uint::zero(width));
        csr[MIMPID] = ArchRegister::new(width, String::from("mimpid"), MIMPID, Uint::zero(width));
        csr[MHARTID] = ArchRegister::new(width, String::from("mhartid"), MHARTID, Uint::zero(width));
        csr[MCONFIGPTR] =
            ArchRegister::new(width, String::from("mconfigptr"), MCONFIGPTR, Uint::zero(width));
        csr[MSTATUS] = ArchRegister::new(width, String::from("mstatus"), MSTATUS, Uint::zero(width));
        if width == 32 {
            csr[MSTATUSH] =
                ArchRegister::new(width, String::from("mstatush"), MSTATUSH, Uint::zero(width));
        }
        csr[MISA] = ArchRegister::new(width, String::from("misa"), MISA, Uint::zero(width));
        csr[MEDELEG] = ArchRegister::new(width, String::from("medeleg"), MEDELEG, Uint::zero(width));
        csr[MIDELEG] = ArchRegister::new(width, String::from("mideleg"), MIDELEG, Uint::zero(width));
        csr[MIE] = ArchRegister::new(width, String::from("mie"), MIE, Uint::zero(width));
        csr[MTVEC] = ArchRegister::new(width, String::from("mtvec"), MTVEC, Uint::zero(width));
        csr[MCOUNTEREN] =
            ArchRegister::new(width, String::from("mcounteren"), MCOUNTEREN, Uint::zero(width));
        csr[MSCRATCH] = ArchRegister::new(width, String::from("mscratch"), MSCRATCH, Uint::zero(width));
        csr[MEPC] = ArchRegister::new(width, String::from("mepc"), MEPC, Uint::zero(width));
        csr[MCAUSE] = ArchRegister::new(width, String::from("mcause"), MCAUSE, Uint::zero(width));
        csr[MTVAL] = ArchRegister::new(width, String::from("mtval"), MTVAL, Uint::zero(width));
        csr[MIP] = ArchRegister::new(width, String::from("mip"), MIP, Uint::zero(width));
        csr[MTINST] = ArchRegister::new(width, String::from("mtinst"), MTINST, Uint::zero(width));
        csr[MTVAL2] = ArchRegister::new(width, String::from("mtval2"), MTVAL2, Uint::zero(width));
        csr[MENVCFG] = ArchRegister::new(width, String::from("menvcfg"), MENVCFG, Uint::zero(width));
        csr[MSECCFG] = ArchRegister::new(width, String::from("mseccfg"), MSECCFG, Uint::zero(width));
        if width == 32 {
            csr[MENVCFGH] =
                ArchRegister::new(width, String::from("menvcfgh"), MENVCFGH, Uint::zero(width));
            csr[MSECCFGH] =
                ArchRegister::new(width, String::from("mseccfgh"), MSECCFGH, Uint::zero(width));
        }
        for i in 0..0x0f {
            csr[PMPCFG0 + i] =
                ArchRegister::new(width, format!("pmpcfg{}", i), PMPCFG0 + i, Uint::zero(width));
            csr[PMPADDR0 + i] =
                ArchRegister::new(width, format!("pmpaddr{}", i), PMPADDR0 + i, Uint::zero(width));
        }
        csr[MNSCRATCH] =
            ArchRegister::new(width, String::from("mnscratch"), MNSCRATCH, Uint::zero(width));
        csr[MNEPC] = ArchRegister::new(width, String::from("mnepc"), MNEPC, Uint::zero(width));
        csr[MNCAUSE] = ArchRegister::new(width, String::from("mncause"), MNCAUSE, Uint::zero(width));
        csr[MNSTATUS] = ArchRegister::new(width, String::from("mnstatus"), MNSTATUS, Uint::zero(width));
        csr[MCYCLE] = ArchRegister::new(width, String::from("mcycle"), MCYCLE, Uint::zero(width));
        csr[MINSTRET] = ArchRegister::new(width, String::from("minstret"), MINSTRET, Uint::zero(width));
        if width == 32 {
            csr[MCYCLEH] = ArchRegister::new(width, String::from("mcycleh"), MCYCLE, Uint::zero(width));
            csr[MINSTRETH] =
                ArchRegister::new(width, String::from("minstreth"), MINSTRET, Uint::zero(width));
        }
        for i in 0..0x1c {
            csr[MHPMCOUNTER3 + i] = ArchRegister::new(
                width,
                format!("mhpmcounter{}", i + 3),
                MHPMCOUNTER3 + i,
                Uint::zero(width),
            );
            if width == 32 {
                csr[MHPMCOUNTER3H + i] = ArchRegister::new(
                    width,
                    format!("mhpmcounterh{}", i + 3),
                    MHPMCOUNTER3H + i,
                    Uint::zero(width),
                );
            }
        }
        csr[MCOUNTINHIBIT] = ArchRegister::new(
            width,
            String::from("mcountinhibit"),
            MCOUNTINHIBIT,
            Uint::zero(width),
        );
        for i in 0..0x1c {
            csr[MHPMEVENT3 + i] = ArchRegister::new(
                width,
                format!("mhpmevent{}", i + 3),
                MHPMEVENT3 + i,
                Uint::zero(width),
            );
        }

        println!("* Populating CSR registers for debug mode");
        csr[TSELECT] = ArchRegister::new(width, String::from("tselect"), TSELECT, Uint::zero(width));
        csr[TDATA1] = ArchRegister::new(width, String::from("tdata1"), TDATA1, Uint::zero(width));
        csr[TDATA2] = ArchRegister::new(width, String::from("tdata2"), TDATA2, Uint::zero(width));
        csr[TDATA3] = ArchRegister::new(width, String::from("tdata3"), TDATA3, Uint::zero(width));
        csr[MCONTEXT] = ArchRegister::new(width, String::from("mcontext"), MCONTEXT, Uint::zero(width));
        csr[DCSR] = ArchRegister::new(width, String::from("dcsr"), DCSR, Uint::zero(width));
        csr[DPC] = ArchRegister::new(width, String::from("dpc"), DPC, Uint::zero(width));
        csr[DSCRATCH0] =
            ArchRegister::new(width, String::from("dscratch0"), DSCRATCH0, Uint::zero(width));
        csr[DSCRATCH1] =
            ArchRegister::new(width, String::from("dscratch1"), DSCRATCH1, Uint::zero(width));

        Csr { width, bank: csr }
    }

    pub fn name(&self, addr: usize) -> &str {
        self.bank[addr].name()
    }

    pub fn set(&mut self, addr: usize, value: &Uint) {
        if write_is_allowed(addr) {
            match addr {
                FFLAGS => {
                    let msk: Uint = Uint::from(0x1fu8).extend(self.width).clone();
                    let fcsr = self.bank[FCSR].get() & !msk.clone() | value.clone() & msk;
                    self.bank[FCSR].set(&fcsr);
                },
                FRM => {
                    let msk: Uint = Uint::from(0x3u8 << 5).extend(self.width).clone();
                    let shift: Uint = Uint::from(5u8).extend(self.width).clone();
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
                    let msk: Uint = Uint::from(0x1fu8).extend(self.width).clone();
                    return Some(self.bank[FCSR].get() & msk);
                },
                FRM => {
                    let msk: Uint = Uint::from(0x3u8 << 5).extend(self.width).clone();
                    let shift: Uint = Uint::from(5u8).extend(self.width).clone();
                    return Some((self.bank[FCSR].get() & msk) >> shift);
                },
                _ => {
                    return Some(self.bank[addr].get());
                },
            }
        }

        Some(Uint::zero(self.width))
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
            "(csr ((width {})\n       (bank: {:?})))\n",
            self.width, self.bank
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
