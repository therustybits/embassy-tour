#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::gpio::{AnyPin, Input, Pin, Pull};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting...");
    let p = embassy_nrf::init(Default::default());
    let button_a = button(p.P0_14.degrade(), "A");
    let button_b = button(p.P0_23.degrade(), "B");
    join(button_a, button_b).await;
}

async fn button(pin: AnyPin, id: &'static str) {
    let mut button = Input::new(pin, Pull::None);
    loop {
        button.wait_for_low().await;
        info!("Button(fut) {} pressed", id);
        Timer::after_millis(200).await;
        button.wait_for_high().await;
        info!("Button {} released", id);
    }
}
