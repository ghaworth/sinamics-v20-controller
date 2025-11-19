# Sinamics V20 Controller

Rust-based controller for Siemens Sinamics V20 inverter drive via Modbus RTU, running on Nordic nRF52840-DK.

## Project Status

🚧 **Work in Progress** - Building towards hardware integration

### Completed
- ✅ Modbus RTU CRC-16 implementation
- ✅ V20 register definitions
- ✅ Automated testing framework

### In Progress
- 🔨 Modbus frame building (read/write registers)
- 🔨 nRF52840 UART integration

### Planned
- Button-based drive control (start/stop/speed control)
- Real-time diagnostics via USB serial
- LED status indicators

## Hardware

- Nordic nRF52840-DK development board
- Siemens Sinamics V20 inverter drive (0.12kW, 240V)
- MAX485 RS-485 transceiver module

## Building
```bash
cargo build
cargo test
```

## License

MIT