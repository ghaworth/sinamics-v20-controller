use sinamics_v20_controller::modbus::{calculate_crc16, build_write_single_register, build_read_holding_registers};

#[test]
fn test_crc16_basic() {
    let data = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x01];
    let crc = calculate_crc16(&data);
    assert_eq!(crc, 0x840A);
}

#[test]
fn test_write_single_register_frame() {
    let frame = build_write_single_register(1, 5, 1);

    assert_eq!(frame[0], 0x01);  // Slave address
    assert_eq!(frame[1], 0x06);  // Function code
    assert_eq!(frame[2], 0x00);  // Register high byte
    assert_eq!(frame[3], 0x05);  // Register low byte
    assert_eq!(frame[4], 0x00);  // Value high byte
    assert_eq!(frame[5], 0x01);  // Value low byte
    assert_eq!(frame.len(), 8);  // Total length with CRC
}

#[test]
fn test_read_holding_registers_frame() {
    let frame = build_read_holding_registers(1, 23, 1);

    assert_eq!(frame[0], 0x01);  // Slave address
    assert_eq!(frame[1], 0x03);  // Function code (read)
    assert_eq!(frame[2], 0x00);  // Register high byte
    assert_eq!(frame[3], 0x17);  // Register low byte (23 = 0x17)
    assert_eq!(frame[4], 0x00);  // Count high byte
    assert_eq!(frame[5], 0x01);  // Count low byte
    assert_eq!(frame.len(), 8);  // Total length with CRC
}