#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use {defmt_rtt as _, esp_backtrace as _};
use bt_hci::controller::ExternalController;
use esp_wifi::ble::controller::BleConnector;
use esp32c3_snippets::ble;

// change to 255 if something is not working
pub const L2CAP_MTU: usize = 251;

#[embassy_executor::task]
async fn run() {
    loop {
        info!("Hello world from embassy using esp-hal-async!");
        // replace the log with blinking led?
        Timer::after(Duration::from_millis(2_000)).await;
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timer1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    spawner.spawn(run()).unwrap();

    let mut bluetooth = peripherals.BT;
    let connector = BleConnector::new(&init, &mut bluetooth);
    let controller: ExternalController<_, 20> = ExternalController::new(connector);
    info!("Starting BLE...");
    ble::run::<_, { L2CAP_MTU }>(controller).await;
}
