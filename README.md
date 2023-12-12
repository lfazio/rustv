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
cargo run -- --arch=rv64im_zicsr_zifencei --binary=../twise/rvmulator/riscv-tests/isa/rv64ui-p-sw.bin
```

# Tests

In order to build riscv-tests/isa, should have first to install some packages:

```sh
sudo apt install picolibc-riscv64-unknown-elf gcc-riscv64-unknown-elf
```

and in CC_OPTIONS of `riscv-tests/isa/Makefile` should must add 

```sh
RISCV_GCC_OPTS ?= -static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles
RISCV_GCC_OPTS += -specs=picolibc.specs
```

And now yo ucan build the tests: do not forget to convert ELF files into binary
```
make -C isa
```