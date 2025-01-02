#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Pull};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting...");
    let p = embassy_nrf::init(Default::default());
    let mut button_a = Input::new(p.P0_14, Pull::None);
    loop {
        button_a.wait_for_low().await;
        info!("Button A pressed");
        button_a.wait_for_high().await;
    }
}
