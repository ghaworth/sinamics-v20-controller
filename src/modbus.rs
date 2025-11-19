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
    let mut frame = Vec::new();
    
    frame.push(slave);                         // Slave address
    frame.push(0x06);                    // Function code (write single register)
    frame.push((register >> 8)  as u8);  // Register address high byte
    frame.push((register & 0xFF) as u8); // Register address low byte
    frame.push((value >> 8) as u8);      // Value high byte
    frame.push((value & 0xFF) as u8);    // Value low byte
    
    let crc = calculate_crc16(&frame);
    frame.push((crc & 0xFF) as u8);
    frame.push((crc >> 8) as u8);
    
    frame
}

pub fn build_read_holding_registers(slave: u8, register: u16, count: u16) -> Vec<u8> {
    let mut frame = Vec::new();

    frame.push(slave);                          // Slave address
    frame.push(0x03);                           // Function code (read holding registers)
    frame.push((register >> 8) as u8);          // Register address high byte
    frame.push((register & 0xFF) as u8);        // Register address low byte
    frame.push((count >> 8) as u8);             // Number of registers high byte
    frame.push((count & 0xFF) as u8);           // Number of registers low byte

    let crc = calculate_crc16(&frame);
    frame.push((crc & 0xFF) as u8);             // CRC low byte
    frame.push((crc >> 8) as u8);               // CRC high byte

    frame
}