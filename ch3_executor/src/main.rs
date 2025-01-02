#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;

/// This macro gives us a thread-mode executor & one task that it spawns onto
/// that executor, which is what you'll probably want most of the time. Roughly
/// expands to:
/// 
/// ``` rust
/// #[embassy_executor::task]
/// async fn __embassy_main(_spawner: Spawner) {
///     info!("Starting...");
/// }
/// #[cortex_m_rt::entry]
/// fn main() -> ! {
///     let mut executor = embassy_executor::Executor::new();
///     let executor = unsafe { __make_static(&mut executor) };
///     executor.run(|spawner| {
///         spawner.must_spawn(__embassy_main(spawner));
///     })
/// }
/// ```
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting...");
    // do stuff...
}
