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

pub const EXT_G: u32 = EXT_I | EXT_M | EXT_A | EXT_D;
