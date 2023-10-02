# baremetal_raspi

Proyecto 2 del curso Introducción a los Sistemas Embebidos.

Programa de Rust para correr bare-metal en una Raspberry Pi.

## Requerimientos

- Instalar el toolchain de `arm-none-eabi`. Específicamente los siguientes
programas:
  - `arm-none-eabi-gdb`
  - `arm-none-eabi-objcopy`
- [Instalar Rust](https://www.rust-lang.org/tools/install)
- Agregar el target de ARM a Rust: `rustup target add armv7a-none-eabi`
- Instalar `cargo-binutils`: 
```
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## Compilación

El siguiente comando compila el ejecutable para la arquitectura `armv7a-none-eabi`:

```
cargo build
```

## Simulación

Se puede simular el procesador a través de gdb, el siguiente comando ejecuta
una sesión de simulación de gdb con el ejecutable:

```
cargo run
```

## Flashear

Así se genera el archivo binario:

```
cargo objcopy -- -O binary boot/kernel7.img
```

Alternativamente existe el siguiente alias para el comando de arriba:
```
cargo flash
```

## binutils

Ejemplos de comandos para inspeccionar binario:

## Object dump
```
cargo objdump -- --disassemble --no-show-raw-insn
```

## Inspeccionar tamaño de secciones
```
cargo size -- -A
```
