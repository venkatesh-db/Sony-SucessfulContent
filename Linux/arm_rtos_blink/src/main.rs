
#![no_std]
#![no_main]


/*

Build for ARM

rustup target add thumbv7em-none-eabihf
cargo build --release --target thumbv7em-none-eabihf


brew install qemu
qemu-system-arm \
  -M stm32-p103 \
  -kernel target/thumbv7em-none-eabihf/release/arm_rtos_blink

*/


use panic_halt as _;
use rtic::app;
use stm32f4xx_hal::{
    gpio::{gpioc::PC13, Output, PushPull},
    prelude::*,
};

#[app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output<PushPull>>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let dp = ctx.device;
        let rcc = dp.RCC.constrain();
        let gpioc = dp.GPIOC.split();

        let led = gpioc.pc13.into_push_pull_output();

        blink::spawn().ok();

        (Shared {}, Local { led })
    }

    #[task(local = [led])]
    fn blink(ctx: blink::Context) {
        ctx.local.led.toggle();
        blink::spawn_after(1.secs()).ok();
    }
}
