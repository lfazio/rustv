#[derive(Debug, Clone)]
pub struct Uint {
    value: Vec<u8>,
}

impl Uint {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }
    
    fn into_u8(self) -> u8 {
        self.value[0]
    }
    
    fn into_u16(self) -> u16 {
        u16::from_le_bytes(self.value.try_into().unwrap())
    }
    
    fn into_u32(self) -> u32 {
        u32::from_le_bytes(self.value.try_into().unwrap())
    }
    
    fn into_u64(self) -> u64 {
        u64::from_le_bytes(self.value.try_into().unwrap())
    }
    
    fn into_u128(self) -> u128 {
        u128::from_le_bytes(self.value.try_into().unwrap())
    }
    
    fn into_i8(self) -> i8 {
        i8::from_le_bytes(self.value.try_into().unwrap())
    }
    
    fn into_i16(self) -> i16 {
        i16::from_le_bytes(self.value.try_into().unwrap())
    }
    
    fn into_i32(self) -> i32 {
        i32::from_le_bytes(self.value.try_into().unwrap())
    }
    
    fn into_i64(self) -> i64 {
        i64::from_le_bytes(self.value.try_into().unwrap())
    }
    
    fn into_i128(self) -> i128 {
        i128::from_le_bytes(self.value.try_into().unwrap())
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
            },
            64 => {
                let u: u64 = u64::from(self.clone()) << (width - bits);
                let i: i64 = u as i64 >> (width - bits);
                Uint::from(i)
            },
            128 => {
                let u: u128 = u128::from(self.clone()) << (width - bits);
                let i: i128 = u as i128 >> (width - bits);
                Uint::from(i)
            },
            _ => panic!("Unsupported Uint width: {}", width),
        }
    }
}

impl From<u8> for Uint {
    fn from(value: u8) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<i8> for Uint {
    fn from(value: i8) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<u16> for Uint {
    fn from(value: u16) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<i16> for Uint {
    fn from(value: i16) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<u32> for Uint {
    fn from(value: u32) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<i32> for Uint {
    fn from(value: i32) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<u64> for Uint {
    fn from(value: u64) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<i64> for Uint {
    fn from(value: i64) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<u128> for Uint {
    fn from(value: u128) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<i128> for Uint {
    fn from(value: i128) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
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
        value.into_u32()
    }
}

impl From<Uint> for u64 {
    fn from(value: Uint) -> Self {
        value.into_u64()
    }
}

impl From<Uint> for u128 {
    fn from(value: Uint) -> Self {
        value.into_u128()
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
        let value = Uint::from(Uint::from(42u8));
        assert_eq!(u8::from(value), 42);
        let value = Uint::from(Uint::from(42u16));
        assert_eq!(u16::from(value), 42);
        let value = Uint::from(Uint::from(42u32));
        assert_eq!(u32::from(value), 42);
        let value = Uint::from(Uint::from(42u64));
        assert_eq!(u64::from(value), 42);
        let value = Uint::from(Uint::from(42u128));
        assert_eq!(u128::from(value), 42);
        let value = Uint::from(Uint::from(42i8));
        assert_eq!(i8::from(value), 42);
        let value = Uint::from(Uint::from(42i16));
        assert_eq!(i16::from(value), 42);
        let value = Uint::from(Uint::from(42i32));
        assert_eq!(i32::from(value), 42);
        let value = Uint::from(Uint::from(42i64));
        assert_eq!(i64::from(value), 42);
        let value = Uint::from(Uint::from(42i128));
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
}
