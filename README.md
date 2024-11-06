# QEMU DXE Core Binaries

## Overview

The main purpose of this repository is to integrate the Rust components and dependencies necessary to build a Rust
DXE Core binary that will be used in QEMU firmware.

Currently, two QEMU platforms are supported. The build commands for each are given below.

Set the `RUSTC_BOOTSTRAP` environment variable to `1` in the terminal used for the build.

- Linux: `export RUSTC_BOOTSTRAP=1`
- Windows (cmd): `set RUSTC_BOOTSTRAP=1`
- Windows (powershell): `$env:RUSTC_BOOTSTRAP=1`

---

- **QEMU Q35**: `cargo build_q35`
  - Release build: `cargo build_q35 --profile=release`
- **QEMU SBSA**: `cargo build_sbsa`
  - Release build: `cargo build_sbsa --profile=release`

The binaries are produced in the `target` directory.

- **QEMU Q35**: `target/x86_64-unknown-uefi`
- **QEMU SBSA**: `target/aarch64-unknown-uefi`

## Working with Local Dependencies

In your development workflow, you should test your firmware changes on QEMU. You can replace the dependencies in this
repo with your local repo for each dependency to build and test that code.

To do that, replace each dependency that you need to substitute with the path to the directory in the local
repository that contains the package manifest for the dependency. For example, in the `Cargo.toml` file contents shown
below, to test a new DXE core, you would replace
\"`git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"`\" with
\"`path = "C:/src/uefi-dxe-core/<dependency_path>"`\" for code in the local directory `C:/src/uefi-dxe-core`:

```toml
section_extractor = {git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"}
serial_writer = {git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"}
uefi_cpu_init = {git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"}
uefi_logger = {git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"}

adv_logger = {git = "https://github.com/pop-project/uefi-dxe-core.git", rev = "dee38a69a6be52a45b3c970c8b83b1af095b610b"}
dxe_core = {git = "https://github.com/pop-project/uefi-dxe-core.git", rev = "dee38a69a6be52a45b3c970c8b83b1af095b610b"}
sample_components = {git = "https://github.com/pop-project/uefi-dxe-core.git", rev = "dee38a69a6be52a45b3c970c8b83b1af095b610b"}

log = { version = "^0.4", default-features = false, features = ["release_max_level_warn"]}
```

To produce the following temporary contents in the `Cargo.toml` file:

```toml
section_extractor = {git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"}
serial_writer = {git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"}
uefi_cpu_init = {git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"}
uefi_logger = {git = "https://github.com/pop-project/uefi-core.git", rev = "891819afcd4bceac694cae1f7339fa09334d9324"}

adv_logger = {path = "C:/src/uefi-dxe-core/adv_logger"}
dxe_core = {path = "C:/src/uefi-dxe-core/dxe_core"}
sample_components = {path = "C:/src/uefi-dxe-core/sample_components"}

log = { version = "^0.4", default-features = false, features = ["release_max_level_warn"]}
```
