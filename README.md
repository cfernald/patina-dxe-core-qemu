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

To do that, follow the [Overriding Dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html)
section in the Cargo Book. Notice that although the `crates-io` registry is replaced with the `UefiRust` in our repo
in `.cargo/config.toml`, the `crates-io` registry is still patched here similar to the examples in the Cargo Book.

```toml
adv_logger = { version = "7" }
dxe_core = { version = "7" }
log = { version = "^0.4", default-features = false, features = [
    "release_max_level_warn",
] }
sample_components = { version = "7" }
section_extractor = { version = "9" }
uefi_cpu = { version = "9" }
uefi_debugger = { version = "9" }
uefi_sdk = { version = "1" }
```

To produce the following temporary contents in the `Cargo.toml` file:

```toml
adv_logger = { version = "7" }
dxe_core = { version = "7" }
log = { version = "^0.4", default-features = false, features = [
    "release_max_level_warn",
] }
sample_components = { version = "7" }
section_extractor = { version = "9" }
uefi_cpu = { version = "9" }
uefi_debugger = { version = "9" }
uefi_sdk = { version = "1" }

[patch.crates-io]
dxe_core = { path = "../uefi-dxe-core/dxe_core" }
```

## NuGet Publishing Instructions

The NuGet package is currently published to [DxeRust](https://dev.azure.com/microsoft/MsUEFI/_artifacts/feed/DxeRust)
feed where it is consumed in the [UefiRust](https://dev.azure.com/microsoft/MsUEFI/_git/UefiRust) repository.

To publish the NuGet package, follow the steps below:

1. Create and activate a virtual environment for this workspace.

    Windows example:

    ```bash
    > py -3 -m venv .venv
    > .venv\Scripts\activate
    ```

2. Install the `edk2-pytool-extensions` pip module.

    ```bash
    > pip install edk2-pytool-extensions
    ```

3. Build the Q35 and SBSA `debug` and `release` binaries.

4. Navigate to the `.nuget` directory of this repo.

5. Create a directory called `DXECORE.QEMU` structured as shown below with the binaries from your build. This is the
   content of the NuGet package.

    ```txt
       DXECORE.QEMU
       ├───debug
       │       qemu_q35_dxe_core.efi
       │       qemu_q35_dxe_core.pdb
       │       qemu_sbsa_dxe_core.efi
       │       qemu_sbsa_dxe_core.pdb
       │
       └───release
               qemu_q35_dxe_core.efi
               qemu_q35_dxe_core.pdb
               qemu_sbsa_dxe_core.efi
               qemu_sbsa_dxe_core.pdb
    ```

6. Run the following (Windows) command.
   - Subsitute `<Version>` with the version number (from Cargo.toml).
   - Substitute `<PAT>` with your Personal Access Token (PAT) from Azure DevOps.
   - Update the `CustomLicensePath` value to an absolute file path.

    ```bash
    nuget-pubish --Operation PackAndPush --OutputLog "NugetLog.txt" --ConfigFilePath dxe_core_config.yaml^
      --InputFolderPath DXECORE.QEMU --Version "<Version>" --ApiKey "<PAT>" --CustomLicensePath license.txt
    ```
