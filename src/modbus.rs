pub fn calculate_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;

    for byte in data {
        crc ^= *byte as u16;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }

    crc.swap_bytes()
}

pub fn build_write_single_register(slave: u8, register: u16, value: u16) -> Vec<u8> {
    vec![]
}

pub fn build_read_holding_registers(slave: u8, register: u16, count: u16) -> Vec<u8> {
    vec![]
}