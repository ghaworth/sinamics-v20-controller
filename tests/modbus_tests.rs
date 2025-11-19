use sinamics_v20_controller::modbus::calculate_crc16;

#[test]
fn test_crc16_basic() {
    let data = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x01];
    let crc = calculate_crc16(&data);
    assert_eq!(crc, 0x840A);
}