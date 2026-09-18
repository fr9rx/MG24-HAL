//! I2C driver, shaped after the `esp-hal` API.
//!
//! Supports both I2C0 and I2C1 interfaces with leader (master) mode.
//! The driver uses builder pattern configuration following esp-hal conventions.
//!
//! Pin assignments (from PINS.md):
//! - I2C0: SDA=PC4, SCL=PC5
//! - I2C1: SDA=PB3, SCL=PB2
//!
//! ```ignore
//! use mg24_hal::{
//!     CpuConfig,
//!     i2c::{I2c, I2cConfig, I2cSpeed, I2c0},
//! };
//!
//! let dp = mg24_hal::init(CpuConfig::default().with_i2c_clock(true)).unwrap();
//! let mut i2c = I2c::new(
//!     dp.pins.pc4,  // SDA
//!     dp.pins.pc5,  // SCL
//!     I2cConfig::default().with_speed(I2cSpeed::Standard),
//! );
//!
//! let mut buf = [0u8; 4];
//! i2c.read(0x50, &mut buf).unwrap();
//! ```
//!
//! Register details follow the EFR32xG24 Reference Manual, chapter 21.

use core::marker::PhantomData;
use core::fmt;
use efr32mg24_pac::{I2c0S, I2c1S};

pub type I2cResult<T> = Result<T, I2cError>;
use embedded_hal::i2c;

use crate::gpio::{AnyPin, Pin};

/// I2C bus speeds (standard mode, fast mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cSpeed {
    /// 100 kHz standard mode
    Standard,
    /// 400 kHz fast mode
    Fast,
}

impl Default for I2cSpeed {
    fn default() -> Self {
        I2cSpeed::Standard
    }
}

/// I2C configuration builder.
#[derive(Debug, Clone)]
pub struct I2cConfig {
    speed: I2cSpeed,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            speed: I2cSpeed::Standard,
        }
    }
}

impl I2cConfig {
    /// Create new I2C configuration.
    pub const fn new() -> Self {
        Self {
            speed: I2cSpeed::Standard,
        }
    }

    /// Set I2C speed.
    pub const fn with_speed(mut self, speed: I2cSpeed) -> Self {
        self.speed = speed;
        self
    }
}

/// I2C0 instance marker.
pub struct I2c0;

/// I2C1 instance marker.
pub struct I2c1;

/// I2C driver errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cError {
    /// Bus arbitration lost
    ArbitrationLost,
    /// NACK received from slave
    NoAck,
    /// Bus error detected
    BusError,
    /// Bus held by another device
    BusHeld,
    /// Timeout waiting for operation
    Timeout,
    /// Invalid address
    InvalidAddress,
}

impl fmt::Display for I2cError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            I2cError::ArbitrationLost => write!(f, "Bus arbitration lost"),
            I2cError::NoAck => write!(f, "NACK received from slave"),
            I2cError::BusError => write!(f, "Bus error detected"),
            I2cError::BusHeld => write!(f, "Bus held by another device"),
            I2cError::Timeout => write!(f, "Timeout waiting for operation"),
            I2cError::InvalidAddress => write!(f, "Invalid I2C address"),
        }
    }
}

/// I2C driver for leader mode (master).
pub struct I2c<'d, T> {
    _sda: AnyPin,
    _scl: AnyPin,
    config: I2cConfig,
    _instance: PhantomData<T>,
    _lifetime: PhantomData<&'d ()>,
}

impl<'d> I2c<'d, I2c0> {
    /// Create a new I2C0 instance in leader mode.
    ///
    /// Configures pins PC4 (SDA) and PC5 (SCL) for I2C0 operation.
    pub fn new(
        sda: impl Pin + 'd,
        scl: impl Pin + 'd,
        config: I2cConfig,
    ) -> Self {
        let _sda = sda.degrade();
        let _scl = scl.degrade();

        unsafe {
            Self::configure_pins(&_sda, &_scl);
            Self::init_peripheral(&config);
        }

        Self {
            _sda,
            _scl,
            config,
            _instance: PhantomData,
            _lifetime: PhantomData,
        }
    }

    /// Configure GPIO pins for I2C0: PC4 (SDA) and PC5 (SCL)
    unsafe fn configure_pins(_sda: &AnyPin, _scl: &AnyPin) { unsafe {
        let gpio = &*efr32mg24_pac::GpioS::ptr();

        // Configure GPIO pins for open-drain with pull-up (WIREDANDPULLUP = 0xA)
        // PC4 and PC5 need to be in open-drain mode for I2C

        // Read current GPIO_PORTC_MODEL value
        let mut model = gpio.portc_model().read().bits();

        // Clear MODE4 and MODE5 (bits [19:16] and [23:20])
        model &= 0xFF00FFFF;

        // Set MODE4 = 0xA (open-drain with pull-up) at bits [19:16]
        // Set MODE5 = 0xA (open-drain with pull-up) at bits [23:20]
        model |= (0xA << 16) | (0xA << 20);

        // Write back the modified MODEL value
        gpio.portc_model().write(|w| w.bits(model));

        // Set DOUT bits 4 and 5 to enable pull-ups
        gpio.portc_dout().modify(|r, w| {
            w.bits(r.bits() | (1 << 4) | (1 << 5))
        });

        // Configure I2C0 GPIO routing
        // Enable I2C0 pin routing
        gpio.i2c0_routeen().write(|w| w.bits(1));

        // Configure SDA: Port C (0x02), Pin 4
        // Route register format: [5:0] = pin, [7:6] = port
        // Port C = 0x02, so: (0x02 << 6) | 4 = (0x80) | 4 = 0x84
        gpio.i2c0_sdaroute().write(|w| w.bits(0x84));

        // Configure SCL: Port C (0x02), Pin 5
        // (0x02 << 6) | 5 = 0x85
        gpio.i2c0_sclroute().write(|w| w.bits(0x85));
    }}

    unsafe fn init_peripheral(config: &I2cConfig) { unsafe {
        let i2c = &*I2c0S::ptr();

        // First, write to the IEN register to ensure no interrupts are enabled initially
        i2c.ien().write(|w| w.bits(0));

        // Enable the I2C peripheral
        i2c.en().write(|w| w.bits(1));

        // Set clock division for desired speed
        // CLKDIV = SYSCLK / (2 * I2C_SCL_FREQ) - 1
        // For 19 MHz and 100 kHz: (19000000 / (2 * 100000)) - 1 = 94
        // For 19 MHz and 400 kHz: (19000000 / (2 * 400000)) - 1 = 23
        let clk_div = match config.speed {
            I2cSpeed::Standard => 94,   // 100 kHz
            I2cSpeed::Fast => 23,       // 400 kHz
        };

        i2c.clkdiv().write(|w| w.div().bits(clk_div));

        // Configure control register for leader mode
        // Bit 2 = AUTOACK (automatic acknowledge)
        // Bit 0 = CORERST (core reset) - must be 0 for normal operation
        // 0x04 = 0b000100 = AUTOACK=1, CORERST=0
        i2c.ctrl().write(|w| w.bits(0x04));

        // Clear any pending interrupt flags
        i2c.if_().write(|w| w.bits(0xFFFF));
    }}

    /// Read bytes from a slave device (leader read operation).
    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2cError> {
        if buffer.is_empty() {
            return Ok(());
        }

        // Validate I2C address (7-bit addressing mode)
        if addr >= 0x80 {
            return Err(I2cError::InvalidAddress);
        }

        let i2c = unsafe { &*I2c0S::ptr() };

        // Issue START command first
        unsafe {
            i2c.cmd().write(|w| w.start().set_bit());
        }

        // Wait for START to be transmitted (state 0x67)
        self.wait_for_state(1_000_000)?;

        // Now write address with read bit to TXDATA
        unsafe {
            let addr_byte = ((addr as u16) << 1) | 1;
            i2c.txdata().write(|w| w.txdata().bits(addr_byte as u8));
        }

        // Wait for address transmission and ACK/NACK
        self.wait_for_state(1_000_000)?;

        // Check for NACK
        if unsafe { i2c.if_().read().nack().bit() } {
            unsafe {
                i2c.if_().write(|w| w.nack().set_bit());
                i2c.cmd().write(|w| w.stop().set_bit());
            }
            return Err(I2cError::NoAck);
        }

        // Read all bytes
        for i in 0..buffer.len() {
            let is_last = i == buffer.len() - 1;

            if is_last {
                // NACK the last byte
                unsafe {
                    i2c.ctrl().modify(|r, w| w.bits(r.bits() | 0x40));
                }
            }

            // Wait for data
            self.wait_for_rxdata(1_000_000)?;

            // Read byte
            buffer[i] = unsafe { i2c.rxdata().read().rxdata().bits() };

            if is_last {
                unsafe {
                    i2c.ctrl().modify(|r, w| w.bits(r.bits() & !0x40));
                }
            }
        }

        // Send STOP
        unsafe { i2c.cmd().write(|w| w.stop().set_bit()) };
        self.wait_for_idle(1_000_000)?;

        Ok(())
    }

    /// Write bytes to a slave device (leader write operation).
    pub fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), I2cError> {
        // Validate I2C address (7-bit addressing mode)
        if addr >= 0x80 {
            return Err(I2cError::InvalidAddress);
        }

        let i2c = unsafe { &*I2c0S::ptr() };

        // Issue START command first
        unsafe {
            i2c.cmd().write(|w| w.start().set_bit());
        }

        // Wait for START to be transmitted (state 0x67)
        self.wait_for_state(1_000_000)?;

        // Now write address with write bit to TXDATA
        unsafe {
            let addr_byte = (addr as u16) << 1;
            i2c.txdata().write(|w| w.txdata().bits(addr_byte as u8));
        }

        // Wait for address transmission and ACK/NACK
        self.wait_for_state(1_000_000)?;

        if unsafe { i2c.if_().read().nack().bit() } {
            unsafe {
                i2c.if_().write(|w| w.nack().set_bit());
                i2c.cmd().write(|w| w.stop().set_bit());
            }
            return Err(I2cError::NoAck);
        }

        // Write all bytes
        for &byte in buffer {
            unsafe { i2c.txdata().write(|w| w.txdata().bits(byte)) };
            self.wait_for_state(1_000_000)?;

            if unsafe { i2c.if_().read().nack().bit() } {
                unsafe {
                    i2c.if_().write(|w| w.nack().set_bit());
                    i2c.cmd().write(|w| w.stop().set_bit());
                }
                return Err(I2cError::NoAck);
            }
        }

        // Send STOP
        unsafe { i2c.cmd().write(|w| w.stop().set_bit()) };
        self.wait_for_idle(1_000_000)?;

        Ok(())
    }

    /// Write then read operation (combined START-RESTART-STOP).
    pub fn write_read(
        &mut self,
        addr: u8,
        write_buffer: &[u8],
        read_buffer: &mut [u8],
    ) -> Result<(), I2cError> {
        // Validate I2C address (7-bit addressing mode)
        if addr >= 0x80 {
            return Err(I2cError::InvalidAddress);
        }

        let i2c = unsafe { &*I2c0S::ptr() };

        // Issue START command first
        unsafe {
            i2c.cmd().write(|w| w.start().set_bit());
        }

        // Wait for START to be transmitted (state 0x67)
        self.wait_for_state(1_000_000)?;

        // Now write address with write bit to TXDATA
        unsafe {
            let addr_byte = (addr as u16) << 1;
            i2c.txdata().write(|w| w.txdata().bits(addr_byte as u8));
        }

        // Wait for address transmission and ACK/NACK
        self.wait_for_state(1_000_000)?;

        if unsafe { i2c.if_().read().nack().bit() } {
            unsafe {
                i2c.if_().write(|w| w.nack().set_bit());
                i2c.cmd().write(|w| w.stop().set_bit());
            }
            return Err(I2cError::NoAck);
        }

        // Write data
        for &byte in write_buffer {
            unsafe { i2c.txdata().write(|w| w.txdata().bits(byte)) };
            self.wait_for_state(1_000_000)?;

            if unsafe { i2c.if_().read().nack().bit() } {
                unsafe {
                    i2c.if_().write(|w| w.nack().set_bit());
                    i2c.cmd().write(|w| w.stop().set_bit());
                }
                return Err(I2cError::NoAck);
            }
        }

        // Send RESTART with read address
        unsafe {
            i2c.cmd().write(|w| w.start().set_bit());
        }

        // Wait for RESTART to be transmitted
        self.wait_for_state(1_000_000)?;

        // Now write address with read bit to TXDATA
        unsafe {
            let addr_byte = ((addr as u16) << 1) | 1;
            i2c.txdata().write(|w| w.txdata().bits(addr_byte as u8));
        }

        // Wait for address transmission and ACK/NACK
        self.wait_for_state(1_000_000)?;

        if unsafe { i2c.if_().read().nack().bit() } {
            unsafe {
                i2c.if_().write(|w| w.nack().set_bit());
                i2c.cmd().write(|w| w.stop().set_bit());
            }
            return Err(I2cError::NoAck);
        }

        // Read data
        if !read_buffer.is_empty() {
            for i in 0..read_buffer.len() {
                let is_last = i == read_buffer.len() - 1;

                if is_last {
                    unsafe {
                        i2c.ctrl().modify(|r, w| w.bits(r.bits() | 0x40));
                    }
                }

                self.wait_for_rxdata(1_000_000)?;
                read_buffer[i] = unsafe { i2c.rxdata().read().rxdata().bits() };

                if is_last {
                    unsafe {
                        i2c.ctrl().modify(|r, w| w.bits(r.bits() & !0x40));
                    }
                }
            }
        }

        // Send STOP
        unsafe { i2c.cmd().write(|w| w.stop().set_bit()) };
        self.wait_for_idle(1_000_000)?;

        Ok(())
    }

    fn wait_for_state(&self, timeout_loops: u32) -> Result<(), I2cError> {
        let i2c = unsafe { &*I2c0S::ptr() };
        for _ in 0..timeout_loops {
            let state = unsafe { i2c.state().read().bits() };
            // Check if STATE (bits [7:5]) == IDLE (0)
            // BUSHOLD bit (4) should also be clear
            if (state & 0xF0) == 0 {
                return Ok(());
            }
        }
        Err(I2cError::Timeout)
    }

    fn wait_for_rxdata(&self, timeout_loops: u32) -> Result<(), I2cError> {
        let i2c = unsafe { &*I2c0S::ptr() };
        for _ in 0..timeout_loops {
            let status = unsafe { i2c.status().read().bits() };
            // Check RXDATAV flag (bit 8)
            if (status & 0x100) != 0 {
                return Ok(());
            }
        }
        Err(I2cError::Timeout)
    }

    fn wait_for_idle(&self, timeout_loops: u32) -> Result<(), I2cError> {
        let i2c = unsafe { &*I2c0S::ptr() };
        for _ in 0..timeout_loops {
            let state = unsafe { i2c.state().read().bits() };
            // Check if STATE (bits [7:5]) == IDLE (0) and BUSHOLD (bit 4) is clear
            if (state & 0xF0) == 0 {
                return Ok(());
            }
        }
        Err(I2cError::Timeout)
    }
}

impl<'d> I2c<'d, I2c1> {
    /// Create a new I2C1 instance in leader mode.
    pub fn new(
        sda: impl Pin + 'd,
        scl: impl Pin + 'd,
        config: I2cConfig,
    ) -> Self {
        let _sda = sda.degrade();
        let _scl = scl.degrade();

        unsafe {
            Self::configure_pins(&_sda, &_scl);
            Self::init_peripheral(&config);
        }

        Self {
            _sda,
            _scl,
            config,
            _instance: PhantomData,
            _lifetime: PhantomData,
        }
    }

    /// Configure GPIO pins for I2C1: PB3 (SDA) and PB2 (SCL)
    unsafe fn configure_pins(_sda: &AnyPin, _scl: &AnyPin) { unsafe {
        let gpio = &*efr32mg24_pac::GpioS::ptr();

        // Configure GPIO pins for open-drain with pull-up (WIREDANDPULLUP = 0xA)
        // PB2 and PB3 need to be in open-drain mode for I2C

        // Read current GPIO_PORTB_MODEL value
        let mut model = gpio.portb_model().read().bits();

        // Clear MODE2 and MODE3 (bits [11:8] and [15:12])
        model &= 0xFFFF00FF;

        // Set MODE2 = 0xA (open-drain with pull-up) at bits [11:8]
        // Set MODE3 = 0xA (open-drain with pull-up) at bits [15:12]
        model |= (0xA << 8) | (0xA << 12);

        // Write back the modified MODEL value
        gpio.portb_model().write(|w| w.bits(model));

        // Set DOUT bits 2 and 3 to enable pull-ups
        gpio.portb_dout().modify(|r, w| {
            w.bits(r.bits() | (1 << 2) | (1 << 3))
        });

        // Configure I2C1 GPIO routing
        // Enable I2C1 pin routing
        gpio.i2c1_routeen().write(|w| w.bits(1));

        // Configure SDA: Port B (0x01), Pin 3
        // Route register format: [5:0] = pin, [7:6] = port
        // Port B = 0x01, so: (0x01 << 6) | 3 = (0x40) | 3 = 0x43
        gpio.i2c1_sdaroute().write(|w| w.bits(0x43));

        // Configure SCL: Port B (0x01), Pin 2
        // (0x01 << 6) | 2 = 0x42
        gpio.i2c1_sclroute().write(|w| w.bits(0x42));
    }}

    unsafe fn init_peripheral(config: &I2cConfig) { unsafe {
        let i2c = &*I2c1S::ptr();

        // First, write to the IEN register to ensure no interrupts are enabled initially
        i2c.ien().write(|w| w.bits(0));

        // Enable the I2C peripheral
        i2c.en().write(|w| w.bits(1));

        // Set clock division for desired speed
        // CLKDIV = SYSCLK / (2 * I2C_SCL_FREQ) - 1
        // For 19 MHz and 100 kHz: (19000000 / (2 * 100000)) - 1 = 94
        // For 19 MHz and 400 kHz: (19000000 / (2 * 400000)) - 1 = 23
        let clk_div = match config.speed {
            I2cSpeed::Standard => 94,   // 100 kHz
            I2cSpeed::Fast => 23,       // 400 kHz
        };

        i2c.clkdiv().write(|w| w.div().bits(clk_div));

        // Configure control register for leader mode
        // Bit 2 = AUTOACK (automatic acknowledge)
        // Bit 0 = CORERST (core reset) - must be 0 for normal operation
        // 0x04 = 0b000100 = AUTOACK=1, CORERST=0
        i2c.ctrl().write(|w| w.bits(0x04));

        // Clear any pending interrupt flags
        i2c.if_().write(|w| w.bits(0xFFFF));
    }}

    /// Read bytes from a slave device.
    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2cError> {
        if buffer.is_empty() {
            return Ok(());
        }

        // Validate I2C address (7-bit addressing mode)
        if addr >= 0x80 {
            return Err(I2cError::InvalidAddress);
        }

        let i2c = unsafe { &*I2c1S::ptr() };

        // Issue START command first
        unsafe {
            i2c.cmd().write(|w| w.start().set_bit());
        }

        // Wait for START to be transmitted (state 0x67)
        self.wait_for_state(1_000_000)?;

        // Now write address with read bit to TXDATA
        unsafe {
            let addr_byte = ((addr as u16) << 1) | 1;
            i2c.txdata().write(|w| w.txdata().bits(addr_byte as u8));
        }

        // Wait for address transmission and ACK/NACK
        self.wait_for_state(1_000_000)?;

        if unsafe { i2c.if_().read().nack().bit() } {
            unsafe {
                i2c.if_().write(|w| w.nack().set_bit());
                i2c.cmd().write(|w| w.stop().set_bit());
            }
            return Err(I2cError::NoAck);
        }

        for i in 0..buffer.len() {
            let is_last = i == buffer.len() - 1;

            if is_last {
                unsafe {
                    i2c.ctrl().modify(|r, w| w.bits(r.bits() | 0x40));
                }
            }

            self.wait_for_rxdata(1_000_000)?;
            buffer[i] = unsafe { i2c.rxdata().read().rxdata().bits() };

            if is_last {
                unsafe {
                    i2c.ctrl().modify(|r, w| w.bits(r.bits() & !0x40));
                }
            }
        }

        unsafe { i2c.cmd().write(|w| w.stop().set_bit()) };
        self.wait_for_idle(1_000_000)?;

        Ok(())
    }

    /// Write bytes to a slave device.
    pub fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), I2cError> {
        // Validate I2C address (7-bit addressing mode)
        if addr >= 0x80 {
            return Err(I2cError::InvalidAddress);
        }

        let i2c = unsafe { &*I2c1S::ptr() };

        // Issue START command first
        unsafe {
            i2c.cmd().write(|w| w.start().set_bit());
        }

        // Wait for START to be transmitted (state 0x67)
        self.wait_for_state(1_000_000)?;

        // Now write address with write bit to TXDATA
        unsafe {
            let addr_byte = (addr as u16) << 1;
            i2c.txdata().write(|w| w.txdata().bits(addr_byte as u8));
        }

        // Wait for address transmission and ACK/NACK
        self.wait_for_state(1_000_000)?;

        if unsafe { i2c.if_().read().nack().bit() } {
            unsafe {
                i2c.if_().write(|w| w.nack().set_bit());
                i2c.cmd().write(|w| w.stop().set_bit());
            }
            return Err(I2cError::NoAck);
        }

        for &byte in buffer {
            unsafe { i2c.txdata().write(|w| w.txdata().bits(byte)) };
            self.wait_for_state(1_000_000)?;

            if unsafe { i2c.if_().read().nack().bit() } {
                unsafe {
                    i2c.if_().write(|w| w.nack().set_bit());
                    i2c.cmd().write(|w| w.stop().set_bit());
                }
                return Err(I2cError::NoAck);
            }
        }

        unsafe { i2c.cmd().write(|w| w.stop().set_bit()) };
        self.wait_for_idle(1_000_000)?;

        Ok(())
    }

    /// Write then read operation.
    pub fn write_read(
        &mut self,
        addr: u8,
        write_buffer: &[u8],
        read_buffer: &mut [u8],
    ) -> Result<(), I2cError> {
        // Validate I2C address (7-bit addressing mode)
        if addr >= 0x80 {
            return Err(I2cError::InvalidAddress);
        }

        let i2c = unsafe { &*I2c1S::ptr() };

        // Issue START command first
        unsafe {
            i2c.cmd().write(|w| w.start().set_bit());
        }

        // Wait for START to be transmitted (state 0x67)
        self.wait_for_state(1_000_000)?;

        // Now write address with write bit to TXDATA
        unsafe {
            let addr_byte = (addr as u16) << 1;
            i2c.txdata().write(|w| w.txdata().bits(addr_byte as u8));
        }

        // Wait for address transmission and ACK/NACK
        self.wait_for_state(1_000_000)?;

        if unsafe { i2c.if_().read().nack().bit() } {
            unsafe {
                i2c.if_().write(|w| w.nack().set_bit());
                i2c.cmd().write(|w| w.stop().set_bit());
            }
            return Err(I2cError::NoAck);
        }

        for &byte in write_buffer {
            unsafe { i2c.txdata().write(|w| w.txdata().bits(byte)) };
            self.wait_for_state(1_000_000)?;

            if unsafe { i2c.if_().read().nack().bit() } {
                unsafe {
                    i2c.if_().write(|w| w.nack().set_bit());
                    i2c.cmd().write(|w| w.stop().set_bit());
                }
                return Err(I2cError::NoAck);
            }
        }

        // Send RESTART with read address
        unsafe {
            i2c.cmd().write(|w| w.start().set_bit());
        }

        // Wait for RESTART to be transmitted
        self.wait_for_state(1_000_000)?;

        // Now write address with read bit to TXDATA
        unsafe {
            let addr_byte = ((addr as u16) << 1) | 1;
            i2c.txdata().write(|w| w.txdata().bits(addr_byte as u8));
        }

        // Wait for address transmission and ACK/NACK
        self.wait_for_state(1_000_000)?;

        if unsafe { i2c.if_().read().nack().bit() } {
            unsafe {
                i2c.if_().write(|w| w.nack().set_bit());
                i2c.cmd().write(|w| w.stop().set_bit());
            }
            return Err(I2cError::NoAck);
        }

        if !read_buffer.is_empty() {
            for i in 0..read_buffer.len() {
                let is_last = i == read_buffer.len() - 1;

                if is_last {
                    unsafe {
                        i2c.ctrl().modify(|r, w| w.bits(r.bits() | 0x40));
                    }
                }

                self.wait_for_rxdata(1_000_000)?;
                read_buffer[i] = unsafe { i2c.rxdata().read().rxdata().bits() };

                if is_last {
                    unsafe {
                        i2c.ctrl().modify(|r, w| w.bits(r.bits() & !0x40));
                    }
                }
            }
        }

        unsafe { i2c.cmd().write(|w| w.stop().set_bit()) };
        self.wait_for_idle(1_000_000)?;

        Ok(())
    }

    fn wait_for_state(&self, timeout_loops: u32) -> Result<(), I2cError> {
        let i2c = unsafe { &*I2c1S::ptr() };
        for _ in 0..timeout_loops {
            let state = i2c.state().read().bits();
            // Check if STATE (bits [7:5]) == IDLE (0)
            // BUSHOLD bit (4) should also be clear
            if (state & 0xF0) == 0 {
                return Ok(());
            }
        }
        Err(I2cError::Timeout)
    }

    fn wait_for_rxdata(&self, timeout_loops: u32) -> Result<(), I2cError> {
        let i2c = unsafe { &*I2c1S::ptr() };
        for _ in 0..timeout_loops {
            let status = i2c.status().read().bits();
            // Check RXDATAV flag (bit 8)
            if (status & 0x100) != 0 {
                return Ok(());
            }
        }
        Err(I2cError::Timeout)
    }

    fn wait_for_idle(&self, timeout_loops: u32) -> Result<(), I2cError> {
        let i2c = unsafe { &*I2c1S::ptr() };
        for _ in 0..timeout_loops {
            let state = i2c.state().read().bits();
            // Check if STATE (bits [7:5]) == IDLE (0) and BUSHOLD (bit 4) is clear
            if (state & 0xF0) == 0 {
                return Ok(());
            }
        }
        Err(I2cError::Timeout)
    }
}

// ========== embedded-hal trait implementations ==========

// Note: embedded-hal 1.0 I2C blocking traits are still in development.
// Currently implement ErrorType which is the foundation trait.
// The driver methods (read, write, write_read) are compatible with
// the embedded-hal I2C blocking interface once finalized.

impl i2c::Error for I2cError {
    fn kind(&self) -> i2c::ErrorKind {
        match self {
            I2cError::ArbitrationLost => i2c::ErrorKind::ArbitrationLoss,
            I2cError::NoAck => i2c::ErrorKind::NoAcknowledge(i2c::NoAcknowledgeSource::Address),
            I2cError::BusError => i2c::ErrorKind::Bus,
            I2cError::BusHeld => i2c::ErrorKind::Bus,
            I2cError::Timeout => i2c::ErrorKind::Other,
            I2cError::InvalidAddress => i2c::ErrorKind::Other,
        }
    }
}

impl i2c::ErrorType for I2c<'_, I2c0> {
    type Error = I2cError;
}

impl i2c::ErrorType for I2c<'_, I2c1> {
    type Error = I2cError;
}

