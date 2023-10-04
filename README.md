# Proyecto 2 Introducción a los Sistemas Embebidos.

## Crates

En este proyecto hay 3 "paquetes" o "crates" como se llaman en Rust:

- [`baremetal-raspi`](./baremetal-raspi): Paquete para ejecutar en Raspberry 3 de manera bare-metal.
- [`nucleo-sensors`](./nucleo-sensors): Paquete para ejecutar en microcrontrolador que lee sensores y los comunica a la Raspberry.
- [`common-types`](./common-types): Biblioteca que contiene tipos que se comunican a través de UART entre el microcontrolador y la Raspbery.

En cada directorio hay un `README.md` con más información.
