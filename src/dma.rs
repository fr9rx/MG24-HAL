//! LDMA (Linked Direct Memory Access) Controller
//!
//! Provides DMA support for memory-to-memory, memory-to-peripheral, and
//! peripheral-to-memory transfers using the EFR32MG24's LDMA controller.
//!
//! # Example
//!
//! ```ignore
//! use mg24_hal::{CpuConfig, dma::{Dma, DmaChannel, DmaConfig, DmaRequest}};
//!
//! let dp = mg24_hal::init(CpuConfig::default()).unwrap();
//! let mut dma = Dma::new();
//!
//! // Configure channel 0 for I2C0 RX
//! let config = DmaConfig::default()
//!     .with_channel(0)
//!     .with_request(DmaRequest::I2c0Rx);
//!
//! // Set up a transfer
//! dma.configure_transfer(
//!     0,
//!     0x4000_a00C as u32,  // I2C0 RXDATA register
//!     &mut buffer[0] as *mut _ as u32,
//!     32,  // 32 bytes to transfer
//!     &config,
//! )?;
//!
//! dma.enable_channel(0)?;
//! dma.start_transfer(0)?;
//!
//! while !dma.is_transfer_done(0) {}
//! dma.clear_done_flag(0)?;
//! ```

use core::fmt;

/// DMA operation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaError {
    /// Invalid channel number (must be 0-7)
    InvalidChannel,
    /// Transfer count is zero or exceeds maximum
    InvalidTransferCount,
    /// Invalid block size
    InvalidBlockSize,
    /// Source and destination addresses overlap
    OverlappingAddresses,
    /// Transfer count exceeds 16-bit limit
    TransferCountTooLarge,
    /// Invalid transfer size configuration
    InvalidSize,
    /// Unsupported operation
    UnsupportedOperation,
}

impl fmt::Display for DmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DmaError::InvalidChannel => write!(f, "Channel must be 0-7"),
            DmaError::InvalidTransferCount => write!(f, "Transfer count must be 1-65536"),
            DmaError::InvalidBlockSize => write!(f, "Invalid block size"),
            DmaError::OverlappingAddresses => write!(f, "Source and destination overlap"),
            DmaError::TransferCountTooLarge => write!(f, "Transfer count exceeds 65536"),
            DmaError::InvalidSize => write!(f, "Invalid transfer size"),
            DmaError::UnsupportedOperation => write!(f, "Unsupported DMA operation"),
        }
    }
}

pub type DmaResult<T> = Result<T, DmaError>;


/// DMA request sources for peripheral triggering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaRequest {
    /// I2C0 receive FIFO data valid
    I2c0Rx,
    /// I2C0 transmit buffer level
    I2c0Tx,
    /// I2C1 receive FIFO data valid
    I2c1Rx,
    /// I2C1 transmit buffer level
    I2c1Tx,
}

impl DmaRequest {
    /// Get the SOURCESEL value for this request
    fn sourcesel(&self) -> u32 {
        match self {
            DmaRequest::I2c0Rx | DmaRequest::I2c0Tx => 0x5,
            DmaRequest::I2c1Rx | DmaRequest::I2c1Tx => 0x6,
        }
    }

    /// Get the SIGSEL value for this request
    fn sigsel(&self) -> u32 {
        match self {
            DmaRequest::I2c0Rx | DmaRequest::I2c1Rx => 0x0,
            DmaRequest::I2c0Tx | DmaRequest::I2c1Tx => 0x1,
        }
    }
}

/// DMA transfer size configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaSize {
    /// Byte (8-bit) transfers
    Byte = 0,
    /// Half-word (16-bit) transfers
    HalfWord = 1,
    /// Word (32-bit) transfers
    Word = 2,
}

/// DMA address increment configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaIncrement {
    /// Increment by 1 unit
    One = 0,
    /// Increment by 2 units
    Two = 1,
    /// Increment by 4 units
    Four = 2,
    /// No increment (fixed address)
    None = 3,
}

/// DMA channel configuration builder
#[derive(Debug, Clone)]
pub struct DmaConfig {
    channel: u8,
    request: Option<DmaRequest>,
    size: DmaSize,
    src_inc: DmaIncrement,
    dst_inc: DmaIncrement,
    block_size: u16,
}

impl DmaConfig {
    /// Set the DMA channel (0-7)
    pub fn with_channel(mut self, ch: u8) -> Self {
        self.channel = ch;
        self
    }

    /// Set the DMA request source
    pub fn with_request(mut self, request: DmaRequest) -> Self {
        self.request = Some(request);
        self
    }

    /// Set the transfer size
    pub fn with_size(mut self, size: DmaSize) -> Self {
        self.size = size;
        self
    }

    /// Set the source address increment
    pub fn with_src_inc(mut self, inc: DmaIncrement) -> Self {
        self.src_inc = inc;
        self
    }

    /// Set the destination address increment
    pub fn with_dst_inc(mut self, inc: DmaIncrement) -> Self {
        self.dst_inc = inc;
        self
    }

    /// Set the block transfer size (units per arbitration cycle)
    pub fn with_block_size(mut self, size: u16) -> Self {
        self.block_size = size;
        self
    }
}

impl Default for DmaConfig {
    fn default() -> Self {
        Self {
            channel: 0,
            request: None,
            size: DmaSize::Byte,
            src_inc: DmaIncrement::One,
            dst_inc: DmaIncrement::One,
            block_size: 1,
        }
    }
}

/// LDMA (Linked Direct Memory Access) Controller
///
/// Provides access to the EFR32MG24's LDMA controller for DMA transfers.
/// The controller has 8 independent channels (0-7) that can be configured
/// for various data transfer scenarios.
pub struct Dma {
    _private: (),
}

impl Dma {
    /// Create a new LDMA controller instance and enable the module
    ///
    /// # Safety
    ///
    /// The LDMA controller must only be instantiated once. Multiple instances
    /// will cause undefined behavior.
    pub fn new() -> Self {
        unsafe {
            // Enable the LDMA module (RM 24.7.2 LDMA_EN)
            let en_ptr = (0x4001_2000 + 0x004) as *mut u32;
            core::ptr::write_volatile(en_ptr, 0x1);
        }

        Self { _private: () }
    }

    /// Copy memory with reference (safe, preferred)
    ///
    /// # Example
    ///
    /// ```ignore
    /// dma.copy_slice(0, &src_data, &mut dst_buffer)?;
    /// while !dma.is_transfer_done(0) {}
    /// ```
    pub fn copy_slice(&mut self, channel: u8, src: &[u8], dst: &mut [u8]) -> DmaResult<()> {
        let count = src.len().min(dst.len()) as u16;
        if count == 0 {
            return Err(DmaError::InvalidTransferCount);
        }
        let src_addr = src.as_ptr() as u32;
        let dst_addr = dst.as_mut_ptr() as u32;
        self.copy_addr(channel, src_addr, dst_addr, count)?;
        Ok(())
    }

    /// Copy memory with addresses (when you have raw pointers)
    ///
    /// # Safety
    ///
    /// Addresses must point to valid memory. Source and destination must not overlap.
    pub fn copy_addr(&mut self, channel: u8, src: u32, dst: u32, count: u16) -> DmaResult<()> {
        let config = DmaConfig::default()
            .with_size(DmaSize::Word)
            .with_src_inc(DmaIncrement::One)
            .with_dst_inc(DmaIncrement::One);
        self.configure_transfer(channel, src, dst, count, &config)?;
        self.enable_channel(channel)?;
        self.start_transfer(channel)?;
        Ok(())
    }

    /// I2C0 RX via DMA with reference (safe, preferred)
    ///
    /// # Example
    ///
    /// ```ignore
    /// dma.i2c0_rx_slice(0, &mut rx_buffer)?;
    /// while !dma.is_transfer_done(0) {}
    /// ```
    pub fn i2c0_rx_slice(&mut self, channel: u8, dst: &mut [u8]) -> DmaResult<()> {
        if dst.is_empty() {
            return Err(DmaError::InvalidTransferCount);
        }
        let count = dst.len() as u16;
        let dst_addr = dst.as_mut_ptr() as u32;
        self.i2c0_rx_addr(channel, dst_addr, count)?;
        Ok(())
    }

    /// I2C0 RX via DMA with address
    pub fn i2c0_rx_addr(&mut self, channel: u8, dst: u32, count: u16) -> DmaResult<()> {
        let config = DmaConfig::default()
            .with_size(DmaSize::Byte)
            .with_src_inc(DmaIncrement::None)
            .with_dst_inc(DmaIncrement::One);
        self.configure_request(channel, DmaRequest::I2c0Rx)?;
        self.configure_transfer(channel, 0x4000_a00C, dst, count, &config)?;
        self.enable_channel(channel)?;
        Ok(())
    }

    /// I2C0 TX via DMA with reference (safe, preferred)
    ///
    /// # Example
    ///
    /// ```ignore
    /// dma.i2c0_tx_slice(0, &tx_buffer)?;
    /// while !dma.is_transfer_done(0) {}
    /// ```
    pub fn i2c0_tx_slice(&mut self, channel: u8, src: &[u8]) -> DmaResult<()> {
        if src.is_empty() {
            return Err(DmaError::InvalidTransferCount);
        }
        let count = src.len() as u16;
        let src_addr = src.as_ptr() as u32;
        self.i2c0_tx_addr(channel, src_addr, count)?;
        Ok(())
    }

    /// I2C0 TX via DMA with address
    pub fn i2c0_tx_addr(&mut self, channel: u8, src: u32, count: u16) -> DmaResult<()> {
        let config = DmaConfig::default()
            .with_size(DmaSize::Byte)
            .with_src_inc(DmaIncrement::One)
            .with_dst_inc(DmaIncrement::None);
        self.configure_request(channel, DmaRequest::I2c0Tx)?;
        self.configure_transfer(channel, src, 0x4000_a008, count, &config)?;
        self.enable_channel(channel)?;
        Ok(())
    }

    /// I2C1 RX via DMA with reference (safe, preferred)
    pub fn i2c1_rx_slice(&mut self, channel: u8, dst: &mut [u8]) -> DmaResult<()> {
        if dst.is_empty() {
            return Err(DmaError::InvalidTransferCount);
        }
        let count = dst.len() as u16;
        let dst_addr = dst.as_mut_ptr() as u32;
        self.i2c1_rx_addr(channel, dst_addr, count)?;
        Ok(())
    }

    /// I2C1 RX via DMA with address
    pub fn i2c1_rx_addr(&mut self, channel: u8, dst: u32, count: u16) -> DmaResult<()> {
        let config = DmaConfig::default()
            .with_size(DmaSize::Byte)
            .with_src_inc(DmaIncrement::None)
            .with_dst_inc(DmaIncrement::One);
        self.configure_request(channel, DmaRequest::I2c1Rx)?;
        self.configure_transfer(channel, 0x4000_a40C, dst, count, &config)?;
        self.enable_channel(channel)?;
        Ok(())
    }

    /// I2C1 TX via DMA with reference (safe, preferred)
    pub fn i2c1_tx_slice(&mut self, channel: u8, src: &[u8]) -> DmaResult<()> {
        if src.is_empty() {
            return Err(DmaError::InvalidTransferCount);
        }
        let count = src.len() as u16;
        let src_addr = src.as_ptr() as u32;
        self.i2c1_tx_addr(channel, src_addr, count)?;
        Ok(())
    }

    /// I2C1 TX via DMA with address
    pub fn i2c1_tx_addr(&mut self, channel: u8, src: u32, count: u16) -> DmaResult<()> {
        let config = DmaConfig::default()
            .with_size(DmaSize::Byte)
            .with_src_inc(DmaIncrement::One)
            .with_dst_inc(DmaIncrement::None);
        self.configure_request(channel, DmaRequest::I2c1Tx)?;
        self.configure_transfer(channel, src, 0x4000_a408, count, &config)?;
        self.enable_channel(channel)?;
        Ok(())
    }

    // Keep old simple names as aliases for backward compatibility
    /// Alias for `copy_addr()` - for backward compatibility
    #[deprecated(since = "2.1.0", note = "use `copy_slice()` or `copy_addr()` instead")]
    pub fn copy(&mut self, channel: u8, src: u32, dst: u32, count: u16) {
        self.copy_addr(channel, src, dst, count).ok();
    }

    /// Alias for `i2c0_rx_addr()` - for backward compatibility
    #[deprecated(since = "2.1.0", note = "use `i2c0_rx_slice()` or `i2c0_rx_addr()` instead")]
    pub fn i2c0_rx(&mut self, channel: u8, dst: u32, count: u16) {
        self.i2c0_rx_addr(channel, dst, count).ok();
    }

    /// Alias for `i2c0_tx_addr()` - for backward compatibility
    #[deprecated(since = "2.1.0", note = "use `i2c0_tx_slice()` or `i2c0_tx_addr()` instead")]
    pub fn i2c0_tx(&mut self, channel: u8, src: u32, count: u16) {
        self.i2c0_tx_addr(channel, src, count).ok();
    }

    /// Alias for `i2c1_rx_addr()` - for backward compatibility
    #[deprecated(since = "2.1.0", note = "use `i2c1_rx_slice()` or `i2c1_rx_addr()` instead")]
    pub fn i2c1_rx(&mut self, channel: u8, dst: u32, count: u16) {
        self.i2c1_rx_addr(channel, dst, count).ok();
    }

    /// Alias for `i2c1_tx_addr()` - for backward compatibility
    #[deprecated(since = "2.1.0", note = "use `i2c1_tx_slice()` or `i2c1_tx_addr()` instead")]
    pub fn i2c1_tx(&mut self, channel: u8, src: u32, count: u16) {
        self.i2c1_tx_addr(channel, src, count).ok();
    }

    /// Configure channel peripheral request source
    ///
    /// This sets up the DMA request selection register to connect a channel
    /// to a peripheral trigger source.
    pub fn configure_request(&mut self, channel: u8, request: DmaRequest) -> DmaResult<()> {
        if channel >= 8 {
            return Err(DmaError::InvalidChannel);
        }

        unsafe {
            // Calculate register offset for this channel
            // LDMAXBAR_CHx_REQSEL is at 0x4001_2004 + (channel * 4)
            let ptr = (0x4001_2004 + (channel as usize * 4)) as *mut u32;

            // Write SOURCESEL and SIGSEL
            // RM 24.9.2 LDMAXBAR_CHx_REQSEL
            let reqsel = (request.sourcesel() << 16) | request.sigsel();
            core::ptr::write_volatile(ptr, reqsel);
        }
        Ok(())
    }

    /// Configure the transfer for a channel
    ///
    /// Sets up source address, destination address, and transfer count.
    /// The transfer is not started until `enable_channel()` and `start_transfer()` are called.
    pub fn configure_transfer(
        &mut self,
        channel: u8,
        src_addr: u32,
        dst_addr: u32,
        xfer_count: u16,
        config: &DmaConfig,
    ) -> DmaResult<()> {
        if channel >= 8 {
            return Err(DmaError::InvalidChannel);
        }
        if xfer_count == 0 {
            return Err(DmaError::InvalidTransferCount);
        }

        unsafe {
            // Calculate base offset for this channel's registers
            // Channels are spaced 0x20 bytes apart (RM 24.6)
            let ch_offset = channel as usize * 0x20;

            // Set source address (offset 0x68 for CH0, +ch_offset)
            let src_ptr = (0x4001_2000 + 0x68 + ch_offset) as *mut u32;
            core::ptr::write_volatile(src_ptr, src_addr);

            // Set destination address (offset 0x6C)
            let dst_ptr = (0x4001_2000 + 0x6C + ch_offset) as *mut u32;
            core::ptr::write_volatile(dst_ptr, dst_addr);

            // Configure control register (offset 0x64)
            // RM 24.7.25 LDMA_CHx_CTRL
            let ctrl_ptr = (0x4001_2000 + 0x64 + ch_offset) as *mut u32;
            let ctrl = (1 << 31) |  // DSTMODE: absolute (0)
                       (0 << 30) |  // SRCMODE: absolute (0)
                       ((config.dst_inc as u32) << 28) |  // DSTINC
                       ((config.size as u32) << 26) |  // SIZE
                       ((config.src_inc as u32) << 24) |  // SRCINC
                       (0 << 23) |  // IGNORESREQ: 0
                       (0 << 22) |  // DECLOOPCNT: 0
                       (0 << 21) |  // REQMODE: BLOCK mode (0)
                       (0 << 20) |  // DONEIEN: 0 (disable interrupt)
                       ((block_size_bits(config.block_size) as u32) << 16) |  // BLOCKSIZE
                       (0 << 15) |  // BYTESWAP: 0
                       (((xfer_count - 1) as u32) << 4) |  // XFERCNT (one less than actual)
                       (0 << 0);  // STRUCTTYPE: TRANSFER (0)

            core::ptr::write_volatile(ctrl_ptr, ctrl);
        }
        Ok(())
    }

    /// Enable a DMA channel
    ///
    /// This marks the channel as ready to accept requests.
    pub fn enable_channel(&mut self, channel: u8) -> DmaResult<()> {
        if channel >= 8 {
            return Err(DmaError::InvalidChannel);
        }

        unsafe {
            // Write to LDMA_CHEN (offset 0x24)
            // RM 24.7.10 LDMA_CHEN
            let chen_ptr = (0x4001_2000 + 0x24) as *mut u32;
            core::ptr::write_volatile(chen_ptr, 1 << channel);
        }
        Ok(())
    }

    /// Disable a DMA channel
    ///
    /// Clears any pending transfers on the channel.
    pub fn disable_channel(&mut self, channel: u8) -> DmaResult<()> {
        if channel >= 8 {
            return Err(DmaError::InvalidChannel);
        }

        unsafe {
            // Write to LDMA_CHDIS (offset 0x28)
            // RM 24.7.11 LDMA_CHDIS
            let chdis_ptr = (0x4001_2000 + 0x28) as *mut u32;
            core::ptr::write_volatile(chdis_ptr, 1 << channel);
        }
        Ok(())
    }

    /// Start a software-triggered DMA transfer
    ///
    /// This initiates a transfer on the specified channel by writing to the
    /// LDMA_SWREQ register.
    pub fn start_transfer(&mut self, channel: u8) -> DmaResult<()> {
        if channel >= 8 {
            return Err(DmaError::InvalidChannel);
        }

        unsafe {
            // Write to LDMA_SWREQ (offset 0x3C)
            // RM 24.7.16 LDMA_SWREQ
            let swreq_ptr = (0x4001_2000 + 0x3C) as *mut u32;
            core::ptr::write_volatile(swreq_ptr, 1 << channel);
        }
        Ok(())
    }

    /// Check if a channel's transfer is complete
    ///
    /// Returns true if the CHDONE flag is set for this channel.
    pub fn is_transfer_done(&self, channel: u8) -> DmaResult<bool> {
        if channel >= 8 {
            return Err(DmaError::InvalidChannel);
        }

        unsafe {
            // Read LDMA_CHDONE (offset 0x34)
            // RM 24.7.14 LDMA_CHDONE
            let chdone_ptr = (0x4001_2000 + 0x34) as *const u32;
            let chdone = core::ptr::read_volatile(chdone_ptr);
            Ok((chdone & (1 << channel)) != 0)
        }
    }

    /// Clear the done flag for a channel
    ///
    /// This must be called after a transfer completes to clear the interrupt flag.
    pub fn clear_done_flag(&mut self, channel: u8) -> DmaResult<()> {
        if channel >= 8 {
            return Err(DmaError::InvalidChannel);
        }

        unsafe {
            // Write to LDMA_CHDONE (offset 0x34)
            // RM 24.7.14 LDMA_CHDONE - writing 1 clears the bit
            let chdone_ptr = (0x4001_2000 + 0x34) as *mut u32;
            core::ptr::write_volatile(chdone_ptr, 1 << channel);
        }
        Ok(())
    }

    /// Check if any channel is currently busy
    pub fn any_busy(&self) -> bool {
        unsafe {
            // Read LDMA_STATUS (offset 0x0C)
            // RM 24.7.4 LDMA_STATUS - ANYBUSY is bit 0
            let status_ptr = (0x4001_2000 + 0x0C) as *const u32;
            let status = core::ptr::read_volatile(status_ptr);
            (status & 0x1) != 0
        }
    }

    /// Get the current status of all channels
    pub fn channel_status(&self) -> u8 {
        unsafe {
            // Read LDMA_CHSTATUS (offset 0x2C)
            // RM 24.7.12 LDMA_CHSTATUS
            let status_ptr = (0x4001_2000 + 0x2C) as *const u32;
            (core::ptr::read_volatile(status_ptr) & 0xFF) as u8
        }
    }
}

/// Convert block size to BLOCKSIZE register encoding
/// RM 24.7.25 LDMA_CHx_CTRL - BLOCKSIZE field
fn block_size_bits(size: u16) -> u8 {
    match size {
        1 => 0x0,
        2 => 0x1,
        3 => 0x2,
        4 => 0x3,
        6 => 0x4,
        8 => 0x5,
        16 => 0x7,
        32 => 0x9,
        64 => 0xA,
        128 => 0xB,
        256 => 0xC,
        512 => 0xD,
        1024 => 0xE,
        _ => 0x0,
    }
}

impl Default for Dma {
    fn default() -> Self {
        Self::new()
    }
}
