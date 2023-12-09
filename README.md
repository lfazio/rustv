# RUSTV: a RISC-V emulator in RUST

This project aims to build a RISC-V emulator at first, but able to be improved with other CPU.

# Architectures

## RISC-V

All architectures supports: Zifencei, Zicsr

- RV64I 
- RV64IM
- RV64E
- RV64EM

# TODO

Read the [TODO](./TODO.md)

# Build

```sh
cargo build
```

# Run

```sh
cargo run -- --arch=RV64IM --binary=../twise/rvmulator/riscv-tests/isa/rv64ui-p-sw.bin
```