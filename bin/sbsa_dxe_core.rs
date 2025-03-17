//! DXE Core Sample AARCH64 Binary for QEMU SBSA
//!
//! ## License
//!
//! Copyright (C) Microsoft Corporation. All rights reserved.
//!
//! SPDX-License-Identifier: BSD-2-Clause-Patent
//!
#![cfg(all(target_os = "uefi", feature = "aarch64"))]
#![no_std]
#![no_main]

use adv_logger::{component::AdvancedLoggerComponent, logger::AdvancedLogger};
use stacktrace::StackTrace;
use core::{ffi::c_void, panic::PanicInfo};
use dxe_core::Core;
use sample_components as sc;
use uefi_sdk::{log::Format, serial::UartPl011};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    if let Err(err) = unsafe { StackTrace::dump() } {
        log::error!("StackTrace: {}", err);
    }

    if uefi_debugger::enabled() {
        uefi_debugger::breakpoint();
    }

    loop {}
}

static LOGGER: AdvancedLogger<UartPl011> = AdvancedLogger::new(
    Format::Standard,
    &[
        ("goblin", log::LevelFilter::Off),
        ("uefi_depex", log::LevelFilter::Off),
        ("gcd_measure", log::LevelFilter::Off),
        ("allocations", log::LevelFilter::Off),
        ("efi_memory_map", log::LevelFilter::Off),
    ],
    log::LevelFilter::Trace,
    UartPl011::new(0x6000_0000),
);

static DEBUGGER: uefi_debugger::UefiDebugger<UartPl011> =
    uefi_debugger::UefiDebugger::new(UartPl011::new(0x6000_0000)).with_default_config(false, true, 0);

#[cfg_attr(target_os = "uefi", export_name = "efi_main")]
pub extern "efiapi" fn _start(physical_hob_list: *const c_void) -> ! {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace)).unwrap();
    let adv_logger_component = AdvancedLoggerComponent::<UartPl011>::new(&LOGGER);
    adv_logger_component.init_advanced_logger(physical_hob_list).unwrap();

    uefi_debugger::set_debugger(&DEBUGGER);

    Core::default()
        .with_cpu_init(uefi_cpu::cpu::aarch64::EfiCpuInitAArch64::default())
        .with_interrupt_manager(uefi_cpu::interrupts::InterruptManagerAArch64::new())
        .with_section_extractor(section_extractor::CompositeSectionExtractor::default())
        .with_interrupt_bases(uefi_cpu::interrupts::InterruptBasesAArch64::new(0x40060000 as _, 0x40080000 as _))
        .init_memory(physical_hob_list) // We can make allocations now!
        .with_config(sc::Name("World")) // Config knob for sc::log_hello
        .with_component(adv_logger_component)
        .with_component(sc::log_hello) // Example of a function component
        .with_component(sc::HelloStruct("World")) // Example of a struct component
        .with_component(sc::GreetingsEnum::Hello("World")) // Example of a struct component (enum)
        .with_component(sc::GreetingsEnum::Goodbye("World")) // Example of a struct component (enum)
        .start()
        .unwrap();

    log::info!("Dead Loop Time");
    loop {}
}
