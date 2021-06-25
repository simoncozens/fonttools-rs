/// Convert an array of bits into an integer
pub fn int_list_to_num(int_list: &[u8]) -> u32 {
    let mut flags = 0;
    for flag in int_list {
        flags |= 1 << flag;
    }
    flags
}

pub struct BitInt {
    data: u32,
}
impl BitInt {
    pub fn new() -> BitInt {
        BitInt { data: 0 }
    }
    pub fn push(&mut self, num: u8) {
        self.data |= 1 << num;
    }
    pub fn to_int(self) -> u32 {
        self.data
    }
}
