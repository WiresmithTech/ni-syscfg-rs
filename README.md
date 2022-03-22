# ni-syscfg-rs
Bindings for the NI System Configuration API in Rust

This repo consists of two crates:

* ni-syscfg-sys which is the simple bindings to the C API provided by NI.
* ni-syscfg which provides a safe wrapper around the C API.

## NI-Syscfg-sys

This crate consists of bindings generated using bindgen.

To simplify the build dependencies these will be statically generated once for each version and this crate will follow NI's version number.

If we need to bump a version due to an internal bug we shall use the bugfix version.

The generation command will be saved as a powershell script and will assume a default installation on Windows at C:\Program Files (x86)\National Instruments\Shared\ExternalCompilerSupport

## ni-syscfg

This create will be written as safe bindings to the API and will follow it's own semantic versioning pattern.

## Contributing

This is at the early stages and we still need to work a few things out. To start a discussion around a bug or desired feature create an issue on github so we can plan how it can work.
