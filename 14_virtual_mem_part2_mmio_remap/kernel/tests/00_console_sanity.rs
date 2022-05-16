// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2022 Andre Richter <andre.o.richter@gmail.com>

//! Console sanity tests - RX, TX and statistics.

#![feature(format_args_nl)]
#![no_main]
#![no_std]

/// Console tests should time out on the I/O harness in case of panic.
mod panic_wait_forever;

use libkernel::{bsp, console, cpu, driver, exception, memory, print};

#[no_mangle]
unsafe fn kernel_init() -> ! {
    use console::console;
    use driver::interface::DriverManager;

    exception::handling_init();

    let phys_kernel_tables_base_addr = match memory::mmu::kernel_map_binary() {
        Err(string) => panic!("Error mapping kernel binary: {}", string),
        Ok(addr) => addr,
    };

    if let Err(e) = memory::mmu::enable_mmu_and_caching(phys_kernel_tables_base_addr) {
        panic!("Enabling MMU failed: {}", e);
    }
    // Printing will silently fail from here on, because the driver's MMIO is not remapped yet.

    memory::mmu::post_enable_init();
    bsp::driver::driver_manager().qemu_bring_up_console();
    // Printing available again from here on.

    // Handshake
    assert_eq!(console().read_char(), 'A');
    assert_eq!(console().read_char(), 'B');
    assert_eq!(console().read_char(), 'C');
    print!("OK1234");

    // 6
    print!("{}", console().chars_written());

    // 3
    print!("{}", console().chars_read());

    // The QEMU process running this test will be closed by the I/O test harness.
    cpu::wait_forever();
}
