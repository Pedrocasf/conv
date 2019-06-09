#[cfg(feature = "LittleEndian")]
pub mod conversion{
    pub fn u8_to_u32(val:&[u8])->u32{
        let b0 = val[0] as u32;
        let b1 = val[1] as u32;
        let b2 = val[2] as u32;
        let b3 = val[3] as u32;
        (b3 << 24) | (b2 << 16) | (b1 << 8) | (b0 << 0) 
    }
    pub fn u8_to_u16(val:&[u8])->u16{
        let b0 = val[0] as u16;
        let b1 = val[1] as u16;
        (b1 << 8) | (b0 << 0) 
    }
    pub fn u32_to_u8(x:u32) -> [u8;4] {
        let b0 : u8 = (x >> 24) as u8;
        let b1 : u8 = (x >> 16) as u8;
        let b2 : u8 = (x >> 8) as u8;
        let b3 : u8 = x as u8;
        [b3, b2, b1, b0]
    }
    pub fn u16_to_u8(x:u16) -> [u8;2] {
        let b0 : u8 = (x >> 8) as u8;
        let b1 : u8 = x as u8;
        [b1, b0]
    }
    pub fn u16_to_u32(val:&[u16])->u32{
        let b0 = val[0] as u32;
        let b1 = val[1] as u32;
        (b1 <<16) | (b0 << 0) 
    }
    pub fn u32_to_u16(x:u32) -> [u16;2] {
        let b0 : u16 = (x >> 16) as u16;
        let b1 : u16 = x as u16;
        [b1, b0]
    }
}
#[cfg(feature = "BigEndian")]
pub mod conversion {
    pub fn u8_to_u32(val:&[u8])->u32{
        let b0 = val[0] as u32;
        let b1 = val[1] as u32;
        let b2 = val[2] as u32;
        let b3 = val[3] as u32;
        (b0 << 24) | (b1 << 16) | (b2 << 8) | (b3 << 0) 
    }
    pub fn u8_to_u16(val:&[u8])->u16{
        let b0 = val[0] as u16;
        let b1 = val[1] as u16;
        (b0 << 8) | (b1 << 0) 
    }
    pub fn u32_to_u8(x:u32) -> [u8;4] {
        let b0 : u8 = (x >> 24) as u8;
        let b1 : u8 = (x >> 16) as u8;
        let b2 : u8 = (x >> 8) as u8;
        let b3 : u8 = x as u8;
        [b0, b1, b2, b3]
    }
    pub fn u16_to_u8(x:u16) -> [u8;2] {
        let b0 : u8 = (x >> 8) as u8;
        let b1 : u8 = x as u8;
        [b0, b1]
    }
    pub fn u16_to_u32(val:&[u16])->u32{
        let b0 = val[0] as u32;
        let b1 = val[1] as u32;
        (b0 <<16) | (b1 << 0) 
    }
    pub fn u32_to_u16(x:u32) -> [u16;2] {
        let b0 : u16 = (x >> 16) as u16;
        let b1 : u16 = x as u16;
        [b0, b1]
    }
}