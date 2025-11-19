// Sinamics V20 Modbus register definitions

// Control registers (write)
pub const CMD_START: u16 = 5;           // Register 40006 (offset 5)
pub const FREQ_REF: u16 = 2;            // Register 40003 (offset 2)
pub const RUN_ENABLE: u16 = 3;          // Register 40004 (offset 3)

// Status registers (read)
pub const FREQ_OUTPUT: u16 = 23;        // Register 40024
pub const SPEED: u16 = 24;              // Register 40025
pub const STOP_RUN: u16 = 34;           // Register 40035