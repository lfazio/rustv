use std::{
    fmt,
    ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr},
};

#[derive(Debug, Clone)]
pub struct Uint {
    value: Vec<u8>,
}

impl Uint {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    pub fn zero(width: usize) -> Self {
        Self {
            value: vec![0u8; width / 8],
        }
    }

    pub fn one(width: usize) -> Self {
        let mut value: Vec<u8> = vec![0u8; width / 8];

        value[0] = 1;
        Self {
            value,
        }
    }

    pub fn ff_ff(width: usize) -> Self {
        Self {
            value: vec![0xffu8; width / 8],
        }
    }

    fn into_u8(self) -> u8 {
        let mut v = self.value.clone();
        if v.len() < 1 {
            v.resize(1, 0);
        } else if v.len() > 1 {
            v.truncate(1);
        }

        u8::from_le_bytes(v.try_into().unwrap())
    }

    fn into_u16(self) -> u16 {
        let mut v = self.value.clone();
        if v.len() < 2 {
            v.resize(2, 0);
        } else if v.len() > 2 {
            v.truncate(2);
        }

        u16::from_le_bytes(v.try_into().unwrap())
    }

    fn into_u32(&mut self) -> u32 {
        let mut v = self.value.clone();
        if v.len() < 4 {
            v.resize(4, 0);
        } else if v.len() > 4 {
            v.truncate(4);
        }

        u32::from_le_bytes(v.try_into().unwrap())
    }

    fn into_u64(self) -> u64 {
        let mut v = self.value.clone();
        if v.len() < 8 {
            v.resize(8, 0);
        } else if v.len() > 8 {
            v.truncate(8);
        }

        u64::from_le_bytes(v.try_into().unwrap())
    }

    fn into_u128(self) -> u128 {
        let mut v = self.value.clone();
        if v.len() < 16 {
            v.resize(16, 0);
        } else if v.len() > 16 {
            v.truncate(16);
        }

        u128::from_le_bytes(v.try_into().unwrap())
    }

    fn into_i8(self) -> i8 {
        let mut v = self.value.clone();
        if v.len() < 1 {
            v.resize(1, 0);
        } else if v.len() > 1 {
            v.truncate(1);
        }

        i8::from_le_bytes(v.try_into().unwrap())
    }

    fn into_i16(self) -> i16 {
        let mut v = self.value.clone();
        if v.len() < 2 {
            v.resize(2, 0);
        } else if v.len() > 2 {
            v.truncate(2);
        }

        i16::from_le_bytes(v.try_into().unwrap())
    }

    fn into_i32(self) -> i32 {
        let mut v = self.value.clone();
        if v.len() < 4 {
            v.resize(4, 0);
        } else if v.len() > 4 {
            v.truncate(4);
        }

        i32::from_le_bytes(v.try_into().unwrap())
    }

    fn into_i64(self) -> i64 {
        let mut v = self.value.clone();
        if v.len() < 8 {
            v.resize(8, 0);
        } else if v.len() > 8 {
            v.truncate(8);
        }

        i64::from_le_bytes(v.try_into().unwrap())
    }

    fn into_i128(self) -> i128 {
        let mut v = self.value.clone();
        if v.len() < 16 {
            v.resize(16, 0);
        } else if v.len() > 16 {
            v.truncate(16);
        }

        i128::from_le_bytes(v.try_into().unwrap())
    }

    pub fn truncate(&mut self, len: usize) -> &mut Self {
        self.value.truncate(len);
        self.value.shrink_to_fit();

        self
    }

    pub fn sextend(&mut self, width: usize, bits: usize) -> Self {
        let value = &mut self.value;

        if width / 8 > value.len() {
            value.resize(width / 8, 0);
        }

        match width {
            32 => {
                let u: u32 = u32::from(self.clone()) << (width - bits);
                let i: i32 = u as i32 >> (width - bits);
                Uint::from(i)
            }
            64 => {
                let u: u64 = u64::from(self.clone()) << (width - bits);
                let i: i64 = u as i64 >> (width - bits);
                Uint::from(i)
            }
            128 => {
                let u: u128 = u128::from(self.clone()) << (width - bits);
                let i: i128 = u as i128 >> (width - bits);
                Uint::from(i)
            }
            _ => panic!("Unsupported Uint width: {}", width),
        }
    }

    pub fn extend(&mut self, width: usize) -> &Self {
        self.extend_with(width, 0)
    }

    pub fn extend_with(&mut self, width: usize, v: u8) -> &Self {
        let value = &mut self.value;

        if width / 8 > value.len() {
            value.resize(width / 8, v);
        }

        self
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .iter()
            .rev()
            .cmp(other.value.iter().rev())
    }
}

impl PartialEq for Uint {
    fn eq(&self, other: &Self) -> bool {
        self.value
            .iter()
            .zip(other.value.iter())
            .all(|(x, y)| x == y)
    }
}

impl PartialOrd for Uint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl BitAnd for Uint {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match self.value.len() {
            4 => Uint::from(u32::from(self).bitand(u32::from(rhs))),
            8 => Uint::from(u64::from(self).bitand(u64::from(rhs))),
            16 => Uint::from(u128::from(self).bitand(u128::from(rhs))),
            _ => panic!("Unsupported width"),
        }
    }
}

impl BitOr for Uint {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match self.value.len() {
            4 => Uint::from(u32::from(self).bitor(u32::from(rhs))),
            8 => Uint::from(u64::from(self).bitor(u64::from(rhs))),
            16 => Uint::from(u128::from(self).bitor(u128::from(rhs))),
            _ => panic!("Unsupported width"),
        }
    }
}

impl Not for Uint {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self.value.len() {
            4 => Uint::from(!u32::from(self)),
            8 => Uint::from(!u64::from(self)),
            16 => Uint::from(!u128::from(self)),
            _ => panic!("Unsupported width"),
        }
    }
}

impl BitXor for Uint {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match self.value.len() {
            4 => Uint::from(u32::from(self).bitxor(u32::from(rhs))),
            8 => Uint::from(u64::from(self).bitxor(u64::from(rhs))),
            16 => Uint::from(u128::from(self).bitxor(u128::from(rhs))),
            _ => panic!("Unsupported width"),
        }
    }
}

impl Add for Uint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self.value.len() {
            4 => Uint::from(u32::from(self).wrapping_add(u32::from(rhs))),
            8 => Uint::from(u64::from(self).wrapping_add(u64::from(rhs))),
            16 => Uint::from(u128::from(self).wrapping_add(u128::from(rhs))),
            _ => panic!("Unsupported width"),
        }
    }
}

impl Shl for Uint {
    type Output = Self;

    fn shl(self, rhs: Uint) -> Self::Output {
        let shift: u32 = u32::from(rhs);
        match self.value.len() {
            4 => Uint::from(u32::from(self).wrapping_shl(shift)),
            8 => Uint::from(u64::from(self).wrapping_shl(shift)),
            16 => Uint::from(u128::from(self).wrapping_shl(shift)),
            _ => panic!("Unsupported width"),
        }
    }
}

impl Shr for Uint {
    type Output = Self;

    fn shr(self, rhs: Uint) -> Self::Output {
        let shift: u32 = u32::from(rhs);
        match self.value.len() {
            4 => Uint::from(i32::from(self).wrapping_shr(shift)),
            8 => Uint::from(i64::from(self).wrapping_shr(shift)),
            16 => Uint::from(i128::from(self).wrapping_shr(shift)),
            _ => panic!("Unsupported width"),
        }
    }
}

impl From<Uint> for Vec<u8> {
    fn from(value: Uint) -> Self {
        value.value
    }
}

impl From<u8> for Uint {
    fn from(value: u8) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<i8> for Uint {
    fn from(value: i8) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<u16> for Uint {
    fn from(value: u16) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<i16> for Uint {
    fn from(value: i16) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<u32> for Uint {
    fn from(value: u32) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<i32> for Uint {
    fn from(value: i32) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<u64> for Uint {
    fn from(value: u64) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<i64> for Uint {
    fn from(value: i64) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<u128> for Uint {
    fn from(value: u128) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<i128> for Uint {
    fn from(value: i128) -> Self {
        Self {
            value: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<Uint> for u8 {
    fn from(value: Uint) -> Self {
        value.into_u8()
    }
}

impl From<Uint> for u16 {
    fn from(value: Uint) -> Self {
        value.into_u16()
    }
}

impl From<Uint> for u32 {
    fn from(value: Uint) -> Self {
        value.clone().into_u32()
    }
}

impl From<&mut Uint> for u32 {
    fn from(value: &mut Uint) -> Self {
        value.into_u32()
    }
}

impl From<Uint> for u64 {
    fn from(value: Uint) -> Self {
        value.into_u64()
    }
}

impl From<&mut Uint> for u64 {
    fn from(value: &mut Uint) -> Self {
        value.clone().into_u64()
    }
}

impl From<Uint> for u128 {
    fn from(value: Uint) -> Self {
        value.into_u128()
    }
}

impl From<&mut Uint> for u128 {
    fn from(value: &mut Uint) -> Self {
        value.clone().into_u128()
    }
}

impl From<Uint> for i8 {
    fn from(value: Uint) -> Self {
        value.into_i8()
    }
}

impl From<Uint> for i16 {
    fn from(value: Uint) -> Self {
        value.into_i16()
    }
}

impl From<Uint> for i32 {
    fn from(value: Uint) -> Self {
        value.into_i32()
    }
}

impl From<Uint> for i64 {
    fn from(value: Uint) -> Self {
        value.into_i64()
    }
}

impl From<Uint> for i128 {
    fn from(value: Uint) -> Self {
        value.into_i128()
    }
}

impl fmt::Display for Uint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value.len() {
            4 => write!(f, "{:#x}", u32::from(self.clone())),
            8 => write!(f, "{:#x}", u64::from(self.clone())),
            16 => write!(f, "{:#x}", u128::from(self.clone())),
            _ => write!(f, "{:?}", self.value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uint_from_num() {
        let value = Uint::from(42);
        assert_eq!(u8::from(value), 42);
        let value = Uint::from(42u16);
        assert_eq!(u16::from(value), 42);
        let value = Uint::from(42u32);
        assert_eq!(u32::from(value), 42);
        let value = Uint::from(42u64);
        assert_eq!(u64::from(value), 42);
        let value = Uint::from(42u128);
        assert_eq!(u128::from(value), 42);
        let value = Uint::from(42i8);
        assert_eq!(i8::from(value), 42);
        let value = Uint::from(42i16);
        assert_eq!(i16::from(value), 42);
        let value = Uint::from(42i32);
        assert_eq!(i32::from(value), 42);
        let value = Uint::from(42i64);
        assert_eq!(i64::from(value), 42);
        let value = Uint::from(42i128);
        assert_eq!(i128::from(value), 42);
    }

    #[test]
    fn test_uint_from_uint() {
        let value = Uint::from(42u8);
        assert_eq!(u8::from(value), 42);
        let value = Uint::from(42u16);
        assert_eq!(u16::from(value), 42);
        let value = Uint::from(42u32);
        assert_eq!(u32::from(value), 42);
        let value = Uint::from(42u64);
        assert_eq!(u64::from(value), 42);
        let value = Uint::from(42u128);
        assert_eq!(u128::from(value), 42);
        let value = Uint::from(42i8);
        assert_eq!(i8::from(value), 42);
        let value = Uint::from(42i16);
        assert_eq!(i16::from(value), 42);
        let value = Uint::from(42i32);
        assert_eq!(i32::from(value), 42);
        let value = Uint::from(42i64);
        assert_eq!(i64::from(value), 42);
        let value = Uint::from(42i128);
        assert_eq!(i128::from(value), 42);
    }

    #[test]
    fn test_uint_sextend() {
        let mut value = Uint::from(42);
        value = value.sextend(64, 32);
        assert_eq!(u64::from(value), 42);

        let mut value = Uint::from(42);
        value = value.sextend(128, 64);
        assert_eq!(u128::from(value), 42);

        let mut value = Uint::from(0x80u8);
        value = value.sextend(32, 8);
        assert_eq!(u32::from(value), 0xffffff80);

        let mut value = Uint::from(0x80u8);
        value = value.sextend(64, 8);
        assert_eq!(u64::from(value), 0xffffffffffffff80);

        let mut value = Uint::from(0x80u8);
        value = value.sextend(128, 8);
        assert_eq!(u128::from(value), 0xffffffffffffffffffffffffffffff80);
    }

    #[test]
    fn test_uint_truncate() {
        let mut value = Uint::from(42);
        value.truncate(32);
        assert_eq!(u32::from(value), 42);

        let mut value = Uint::from(42);
        value.truncate(64);
        assert_eq!(u64::from(value), 42);

        let mut value = Uint::from(42);
        value.truncate(128);
        assert_eq!(u128::from(value), 42);

        let mut value = Uint::from(0x80u8);
        value.truncate(8);
        assert_eq!(u8::from(value), 0x80);

        let mut value = Uint::from(0x80u8);
        value.truncate(32);
        assert_eq!(u32::from(value), 0x80);

        let mut value = Uint::from(0x80u8);
        value.truncate(64);
        assert_eq!(u64::from(value), 0x80);

        let mut value = Uint::from(0x80u8);
        value.truncate(128);
        assert_eq!(u128::from(value), 0x80);
    }

    #[test]
    fn test_partial_eq() {
        let value = Uint::from(256u32);

        assert_eq!(value, Uint::from(256u32));
        assert_ne!(value, Uint::from(257u32));

        assert_eq!(value, Uint::from(256u32));
        assert_ne!(value, Uint::from(0xfffffu32));
    }

    #[test]
    fn test_partial_ord_ge() {
        let value = Uint::from(256u32);

        assert!(value >= Uint::from(255u32));
        assert!(value >= Uint::from(254u32));
        assert!(value >= Uint::from(253u32));
        assert!(value >= Uint::from(252u32));
        assert!(value >= Uint::from(251u32));
        assert!(value >= Uint::from(250u32));
        assert!(value >= Uint::from(249u32));
        assert!(value >= Uint::from(248u32));
        assert!(value >= Uint::from(247u32));
        assert!(value >= Uint::from(246u32));
        assert!(value >= Uint::from(245u32));
        assert!(value >= Uint::from(244u32));
        assert!(value >= Uint::from(243u32));
        assert!(value >= Uint::from(242u32));
        assert!(value >= Uint::from(241u32));
        assert!(value >= Uint::from(240u32));
        assert!(value >= Uint::from(239u32));
        assert!(value >= Uint::from(238u32));
        assert!(value >= Uint::from(237u32));
        assert!(value >= Uint::from(236u32));
    }

    #[test]
    fn test_partial_ord_lt() {
        let value = Uint::from(255u32);

        assert!(value < Uint::from(256u32));
        assert!(value < Uint::from(257u32));
        assert!(value < Uint::from(0xfffffu32));
    }

    #[test]
    fn test_add() {
        let value = Uint::from(42u32) + Uint::from(42u32);
        assert_eq!(u32::from(value), 84);

        let value = Uint::from(42u64) + Uint::from(42u64);
        assert_eq!(u64::from(value), 84);

        let value = Uint::from(42u128) + Uint::from(42u128);
        assert_eq!(u128::from(value), 84);

        let value = Uint::from(42i32) + Uint::from(42i32);
        assert_eq!(i32::from(value), 84);

        let value = Uint::from(42i64) + Uint::from(42i64);
        assert_eq!(i64::from(value), 84);

        let value = Uint::from(42i128) + Uint::from(42i128);
        assert_eq!(i128::from(value), 84);

        let value = Uint::from(255u32) + Uint::from(1u32);
        assert_eq!(u32::from(value), 256u32);
    }
}
