pub struct RIA2 {
    pub registers: [u8; 16],
    pub position_in_memory: usize,
    pub memory: [u8; 0x1000],
}

