use std::fmt;

pub const EXT_A: u32 = 1 << 0;
pub const EXT_C: u32 = 1 << 2;
pub const EXT_D: u32 = 1 << 3;
pub const EXT_E: u32 = 1 << 4;
pub const EXT_F: u32 = 1 << 5;
pub const EXT_H: u32 = 1 << 7; /* Hypervisor Mode */
pub const EXT_I: u32 = 1 << 8;
pub const EXT_M: u32 = 1 << 12;
pub const EXT_Q: u32 = 1 << 16;
pub const EXT_S: u32 = 1 << 18; /* Supervisor Mode */
pub const EXT_U: u32 = 1 << 20; /* User Mode */
pub const EXT_V: u32 = 1 << 21;
pub const EXT_X: u32 = 1 << 23;

#[derive(Debug, Default)]
pub struct RvExtensions {
    pub i: bool,
    pub e: bool,
    pub m: bool,
    pub f: bool,
    pub d: bool,
    pub s: bool,
    pub h: bool,
    pub u: bool,
    pub c: bool,
    pub zalrsc: bool, // a
    pub zamo: bool, // a
    pub zacas: bool, // a
    pub zicsr: bool,
    pub zifencei: bool,
    pub zmmul: bool,
    pub zicntr: bool,
    pub zihpm: bool,
}

impl fmt::Display for RvExtensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = writeln!(f, "(extensions");

        if self.i {
            _ = writeln!(f, "     (i\t{})", self.i);
        }

        if self.e {
            _ = writeln!(f, "     (e\t{})", self.e);
        }

        if self.m {
            _ = writeln!(f, "     (m\t{})", self.m);
        }

        if self.f {
            _ = writeln!(f, "     (f\t{})", self.f);
        }

        if self.d {
            _ = writeln!(f, "     (d\t{})", self.d);
        }

        if self.c {
            _ = writeln!(f, "     (c\t{})", self.c);
        }

        if self.s {
            _ = writeln!(f, "     (s\t{})", self.s);
        }

        if self.h {
            _ = writeln!(f, "     (h\t{})", self.h);
        }

        if self.d {
            _ = writeln!(f, "     (u\t{})", self.u);
        }

        if self.zalrsc {
            _ = writeln!(f, "     (zalrsc\t{})", self.zalrsc);
        }

        if self.zamo {
            _ = writeln!(f, "     (zamo\t{})", self.zamo);
        }

        if self.zacas {
            _ = writeln!(f, "     (zacas\t{})", self.zacas);
        }

        if self.zicsr {
            _ = writeln!(f, "     (zicsr\t{})", self.zicsr);
        }

        if self.zifencei {
            _ = writeln!(f, "     (zifencei\t{})", self.zifencei);
        }

        if self.zmmul {
            _ = writeln!(f, "     (zmmul\t{})", self.zmmul);
        }

        if self.zicntr {
            _ = writeln!(f, "     (zicntr\t{})", self.zicntr);
        }

        if self.zihpm {
            _ = writeln!(f, "     (zihpm\t{})", self.zihpm);
        }

        write!(f, "    )")
    }
}
