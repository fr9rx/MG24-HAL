#![no_std]
#![no_main]

use mg24_hal::{CpuConfig, dma::Dma};

#[mg24_hal::main]
fn main() -> ! {
    let _dp = mg24_hal::init(CpuConfig::default()).unwrap();

    let mut dma = Dma::new();

    // Example: Memory-to-memory DMA transfer
    let source = [1u8, 2, 3, 4];
    let mut destination = [0u8; 4];

    if let Ok(()) = dma.copy_slice(0, &source, &mut destination) {
        // Wait for transfer to complete
        while !dma.is_transfer_done(0).unwrap_or(false) {
            // Transfer in progress
        }
    }

    // destination now contains [1, 2, 3, 4]
    loop {}
}
