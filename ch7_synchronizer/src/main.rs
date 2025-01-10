#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::{bind_interrupts, gpio::{AnyPin, Input, Pin, Pull}, temp::Temp};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use embassy_time::{Duration, Timer, WithTimeout};
use panic_probe as _;

#[derive(Clone, Copy)]
enum Button {
    A,
    B,
}

static SIGNAL: Signal<ThreadModeRawMutex, Button> = Signal::new();

bind_interrupts!(struct Irqs {
    TEMP => embassy_nrf::temp::InterruptHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting...");
    let p = embassy_nrf::init(Default::default());
    let temp = Temp::new(p.TEMP, Irqs);
    spawner.spawn(temp_task(temp)).unwrap();
    let button_a = button(p.P0_14.degrade(), "A", Button::A);
    let button_b = button(p.P0_23.degrade(), "B", Button::B);
    join(button_a, button_b).await;
}

async fn button(pin: AnyPin, id: &str, b: Button) {
    let mut button = Input::new(pin, Pull::None);
    loop {
        button.wait_for_low().await;
        info!("Button {} pressed (fut)", id);
        SIGNAL.signal(b);
        Timer::after_millis(200).await;
        button.wait_for_high().await;
    }
}

#[embassy_executor::task]
async fn temp_task(mut temp: Temp<'static>) {
    const INTERVAL_MS: u64 = 500;
    let mut delay_ms = INTERVAL_MS;
    loop {
        let value = temp.read().await.to_num::<u16>();
        info!("{} C", value);
        let delay = Duration::from_millis(delay_ms);
        if let Some(v) = SIGNAL.wait().with_timeout(delay).await.ok() {
            delay_ms = match v {
                Button::A if delay_ms > INTERVAL_MS => delay_ms - INTERVAL_MS,
                Button::A => delay_ms,
                Button::B => delay_ms + INTERVAL_MS,
            };
            info!("Delay = {} ms", delay_ms);
        }
    }
}
