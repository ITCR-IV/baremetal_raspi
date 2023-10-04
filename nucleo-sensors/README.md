# nucleo-sensors

Proyecto generado con el [template de hello-nucleo-h7xx](https://github.com/antoinevg/hello-nucleo-h7xx).

Parte del proyecto 2 del curso Introducción a los Sistemas Embebidos.

Programa de Rust para leer un sensor y comunicarlo por UART en una tarjeta Nucleo-H745ZI-Q.

## Requerimientos

### Instalar target ARM

    rustup target add thumbv7em-none-eabihf

### Instalar probe-run

    cargo install probe-run --version "~0.2.0"

### Instalar firmware para STLink V3

Download updater: [stsw-link007](https://www.st.com/content/st_com/en/products/development-tools/software-development-tools/stm32-software-development-tools/stm32-programmers/stsw-link007.html)

Rust [`probe-run`] + [`defmt`] template para STMicroElectronics [STM32H7 Nucleo-144 boards](https://www.st.com/content/st_com/en/search.html#q=nucleo-h7-t=tools-page=1).

[`probe-run`]: https://crates.io/crates/probe-run
[`defmt`]: https://github.com/knurling-rs/defmt

## Licencia

Licenciado bajo

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
