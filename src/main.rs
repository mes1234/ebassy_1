#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, gpio::Pin, peripherals, twim, uarte};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::PubSubChannel;
use embassy_time::Timer;

use rtt_target::{rprintln, rtt_init_print};

mod drivers;
use drivers::led_driver::led_pin;
use drivers::pwm_driver::pwm_init;
use drivers::servo_driver::servo_driver;
use drivers::uart_driver::uart_init;
use drivers::uart_driver::uart_reader_driver;

mod utils;
use utils::config::configuration_handler;

mod common;
use common::contracts::{Config, ServoSetup};

use panic_probe as _;

bind_interrupts!(struct Irqs {
    UARTE0_UART0 => uarte::InterruptHandler<peripherals::UARTE0>;
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0  => twim::InterruptHandler<peripherals::TWISPI0>;
});

static SERVO_SETUP_CHANNEL: PubSubChannel<ThreadModeRawMutex, ServoSetup, 10, 1, 1> =
    PubSubChannel::new();

static CONFIG_CHANNEL: PubSubChannel<ThreadModeRawMutex, Config, 10, 1, 1> = PubSubChannel::new();

static CONFIG: Mutex<ThreadModeRawMutex, Config> = Mutex::new(Config::default());

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();

    rprintln!("System Booting...");

    let p = embassy_nrf::init(Default::default());

    rprintln!("System Booting: Peripherals init: OK");
    let _row1 = led_pin(p.P0_21.degrade());
    let mut _col1 = led_pin(p.P0_28.degrade());
    let mut _col2 = led_pin(p.P0_11.degrade());
    let mut _col3 = led_pin(p.P0_31.degrade());

    rprintln!("System Booting: LED init: OK");

    let pwm = pwm_init(p.TWISPI0, p.P1_00.into(), p.P0_26.into(), Irqs);
    let (_, rx) = uart_init(p.UARTE0, p.P1_08.into(), p.P0_06.into(), Irqs);

    rprintln!("System Booting: All drivers init: OK");

    let servo_publisher = SERVO_SETUP_CHANNEL.publisher().unwrap();
    let servo_subscriber = SERVO_SETUP_CHANNEL.subscriber().unwrap();

    let config_publisher = CONFIG_CHANNEL.publisher().unwrap();
    let config_subscriber = CONFIG_CHANNEL.subscriber().unwrap();

    let _ = spawner
        .spawn(uart_reader_driver(servo_publisher, config_publisher, rx))
        .unwrap();
    let _ = spawner.spawn(servo_driver(servo_subscriber, pwm, &CONFIG));
    let _ = spawner.spawn(configuration_handler(config_subscriber, &CONFIG));

    loop {
        Timer::after_millis(100).await;
        _col1.set_low();
        Timer::after_millis(100).await;
        _col1.set_high();
    }
}
