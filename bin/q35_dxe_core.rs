//! DXE Core Sample X64 Binary for QEMU Q35
//!
//! ## License
//!
//! Copyright (C) Microsoft Corporation. All rights reserved.
//!
//! SPDX-License-Identifier: BSD-2-Clause-Patent
//!
#![cfg(all(target_os = "uefi", feature = "x64"))]
#![no_std]
#![no_main]

extern crate alloc;

use adv_logger::{component::AdvancedLoggerComponent, logger::AdvancedLogger};
use alloc::boxed::Box;
use core::{ffi::c_void, panic::PanicInfo};
use dxe_core::Core;
use sample_components::HelloComponent;
use uefi_sdk::{log::Format, serial::Uart16550};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    if uefi_debugger::enabled() {
        uefi_debugger::breakpoint();
    }

    loop {}
}

static LOGGER: AdvancedLogger<Uart16550> = AdvancedLogger::new(
    Format::Standard,
    &[
        ("goblin", log::LevelFilter::Off),
        ("uefi_depex", log::LevelFilter::Off),
        ("gcd_measure", log::LevelFilter::Off),
        ("allocations", log::LevelFilter::Off),
        ("efi_memory_map", log::LevelFilter::Off),
    ],
    log::LevelFilter::Trace,
    Uart16550::new(uefi_sdk::serial::Interface::Io(0x402)),
);

static DEBUGGER: uefi_debugger::UefiDebugger<uefi_sdk::serial::Uart16550> =
    uefi_debugger::UefiDebugger::new(uefi_sdk::serial::Uart16550::new(uefi_sdk::serial::Interface::Io(0x3F8)))
        .with_default_config(false, true, 0)
        .with_debugger_logging();

#[cfg_attr(target_os = "uefi", export_name = "efi_main")]
pub extern "efiapi" fn _start(physical_hob_list: *const c_void) -> ! {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace)).unwrap();
    let adv_logger_component = AdvancedLoggerComponent::<Uart16550>::new(&LOGGER);
    adv_logger_component.init_advanced_logger(physical_hob_list).unwrap();

    uefi_debugger::set_debugger(&DEBUGGER);

    Core::default()
        .with_cpu_init(uefi_cpu::cpu::EfiCpuInitX64::default())
        .with_interrupt_manager(uefi_cpu::interrupts::InterruptManagerX64::default())
        .with_interrupt_bases(uefi_cpu::interrupts::InterruptBasesNull::default())
        .with_section_extractor(section_extractor::CompositeSectionExtractor::default())
        // Add any config knob functions for pre-gcd-init Core
        // .with_some_config(true)
        .initialize(physical_hob_list) // We can make allocations now!
        // Add any config knob functions for post-gcd-init Core
        // .with_some_config(true)
        .with_driver(Box::new(adv_logger_component))
        .with_driver(Box::new(HelloComponent::default()))
        .with_driver(Box::new(HelloComponent::default().with_name("Dxe Core")))
        .with_driver(Box::new(HelloComponent::default().with_name("World")))
        .start()
        .unwrap();

    log::info!("Dead Loop Time");
    loop {}
}
