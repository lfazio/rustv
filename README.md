# RUSTV: a RISC-V emulator in RUST

This project aims to build a RISC-V emulator at first, but able to be improved with other CPU.

# Architectures

## RISC-V

Available architectures:
- `rv32i` or `rv32e`
- `rv64i` or `rv64e`
- `rv128i`

> **NOTE**: `rv<xlen>ima` is fully supported

> **NOTE**: `rv<xlen>imafd` is partially supported (only load/store FP instructions are implemented currently)

All architectures supports: 
- `zifencei`
- `zicsr`
- `zmmul`
- `zalrsc` (included in `A`)
- `zamo` (included in `A`)
- `zacas` (included in `A`)

# TODO

Read the [TODO](./TODO.md)

# Build

```sh
cargo build
```

# Run

```sh
cargo run -- --arch=rv64imafd_zicsr_zifencei --binary=../twise/rvmulator/riscv-tests/isa/rv64ui-p-sw.bin
```

Or for example:

```sh
cargo run -- --arch=rv64imfd_zacas_zamo_zalrsc_zicsr_zifencei --binary=../twise/rvmulator/riscv-tests/isa/rv64ua-p-lrsc.bin
```

>
> `a` is equivalent to `_zacas_zamo_zalrsc`
>
> `_zmmul` is also available in order to emulate the subset of the RISC-V ISA you need
>

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