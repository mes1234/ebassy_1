#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{AnyPin, Level, Output, OutputDrive, Pin},
    peripherals,
    twim::{self, Twim},
    uarte,
};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::pubsub::PubSubChannel;
use embassy_time::Timer;
use pwm_pca9685::{Address, Pca9685};

use rtt_target::{rprintln, rtt_init_print};

mod drivers;
use drivers::servo_driver::servo_driver;
use drivers::uart_reader_driver::uart_reader_driver;

mod common;
use common::contracts::ServoSetup;

use panic_probe as _;

bind_interrupts!(struct Irqs {
    UARTE0_UART0 => uarte::InterruptHandler<peripherals::UARTE0>;
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0  => twim::InterruptHandler<peripherals::TWISPI0>;
});

static SERVO_SETUP_CHANNEL: PubSubChannel<ThreadModeRawMutex, ServoSetup, 10, 1, 1> =
    PubSubChannel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();

    rprintln!("Yello");

    let p = embassy_nrf::init(Default::default());

    // LED init
    let _row1 = led_pin(p.P0_21.degrade());
    let mut _col1 = led_pin(p.P0_28.degrade());
    let mut _col2 = led_pin(p.P0_11.degrade());
    let mut _col3 = led_pin(p.P0_31.degrade());

    // PCA9685 init

    let pca9685_address = Address::default();

    let twim_config = twim::Config::default();

    let twim_device = Twim::new(
        p.TWISPI0,
        Irqs,
        p.P1_00.degrade(),
        p.P0_26.degrade(),
        twim_config,
    );

    let mut pwm = Pca9685::new(twim_device, pca9685_address).unwrap();

    pwm.set_prescale(100).unwrap();

    pwm.enable().unwrap();

    // UART init

    let mut uart_config = uarte::Config::default();

    uart_config.parity = uarte::Parity::EXCLUDED;
    uart_config.baudrate = uarte::Baudrate::BAUD38400;

    let uart = uarte::Uarte::new(
        p.UARTE0,
        Irqs,
        p.P1_08.degrade(),
        p.P0_06.degrade(),
        uart_config,
    );

    let (mut tx, rx) = uart.split();

    let publisher = SERVO_SETUP_CHANNEL.publisher().unwrap();
    let mut sub = SERVO_SETUP_CHANNEL.subscriber().unwrap();

    let _ = spawner.spawn(uart_reader_driver(publisher, rx)).unwrap();
    // _col2.set_low();
    // _col3.set_low();

    let _ = spawner.spawn(servo_driver(sub, pwm));

    loop {
        Timer::after_millis(100).await;
        _col1.set_low();
        Timer::after_millis(100).await;
        _col1.set_high();
    }
}

fn led_pin(pin: AnyPin) -> Output<'static> {
    Output::new(pin, Level::High, OutputDrive::Standard)
}
