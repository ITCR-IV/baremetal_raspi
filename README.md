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

Para crear la imagen que se flashea en la tarjeta SD de la raspberry pi se requiere primero compilar, se recomiendo en release mode:

TODO: Ignorar esto de release, solo ejecutar el comando de abajo que agarra el
target de debug, no sé por qué cargo build --release parece generar una imagen
directamente en vez de un ELF?
```
cargo build --release
```

Luego ejecutar:

```
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/release/baremetal_raspi ./kernel.img
```
