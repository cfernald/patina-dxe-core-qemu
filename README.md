# QEMU DXE Core Binaries

## Overview

The main purpose of this repository is to integrate the Rust components and dependencies necessary to build a sample
Rust DXE Core binary that can be used in a QEMU UEFI firmware build.

Currently, two QEMU platforms are supported, Q35 for x64 architecture and SBSA for aarch64 architecture.

To build an executable, this repo uses the same compiler setup steps that are used in the patina project
[readme.md file build section](https://github.com/OpenDevicePartnership/patina#Build).  Once the compiler is installed,
executing cargo make will create a DXE core .EFI file that is a replacement for the dxe core driver in the
[patina-qemu](https://github.com/OpenDevicePartnership/patina-qemu) UEFI build.

- Q35 (x64) debug

   ```shell
   Compile Command:  'cargo make q35'
   Output File:      'target/x86_64-unknown-uefi/debug/qemu_q35_dxe_core.efi'
   ```

- Q35 (x64) release

   ```shell
   Compile Command:  'cargo make q35-release'
   Output File:      'target/x86_64-unknown-uefi/release/qemu_q35_dxe_core.efi'
   ```

- SBSA (aarch64) debug

   ```shell
   Compile Command:  'cargo make sbsa'
   Output File:      'target/aarch64-unknown-uefi/debug/qemu_sbsa_dxe_core.efi'
   ```

- SBSA (aarch64) release

   ```shell
   Compile Command:  'cargo make sbsa-release'
   Output File:      'target/aarch64-unknown-uefi/release/qemu_sbsa_dxe_core.efi'
   ```

## Working with Local Dependencies

If working with local dependencies outside of this repository, such as making changes in [Patina](https://github.com/OpenDevicePartnership/patina)
that you wish to compile into one of the qemu binaries in this repository, then simply add the path to the local
repository to the command line, and the build tools will automatically patch in all crates in that repository for that
build.

``` cmd
> cargo make q35 C:\\src\\patina\\
> cargo make sbsa C:/src/patina C:/src/patina-paging
```

**IMPORTANT**: This tool temporarily adds the patches to the Cargo.toml, so you must meet Cargo.toml expectations
with the path that you define. That is to say, if you are providing windows pathing, you must use double slashes
(`\\`).

## NuGet Publishing Instructions

The NuGet package is currently published to the public [Patina QEMU DXE Core](https://dev.azure.com/patina-fw/artifacts/_artifacts/feed/qemu-dxe-core)
feed where it is consumed in the [Patina QEMU](https://github.com/OpenDevicePartnership/patina-qemu) repository.

The NuGet is built and published using a GitHub workflow in [Patina QEMU DXE Core Actions](https://github.com/OpenDevicePartnership/patina-dxe-core-qemu/actions).
