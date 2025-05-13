#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{AnyPin, Level, Output, OutputDrive, Pin},
    peripherals,
    twim::{self, Twim},
    uarte::{self, UarteRx},
};
use embassy_sync::pubsub::{self, PubSubChannel, Publisher};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, pubsub::Subscriber};
use embassy_time::Timer;
use pwm_pca9685::{Address, Channel, Pca9685};

use cobs::decode_in_place;
use heapless::Vec;

use rtt_target::{
    debug_rprintln, debug_rtt_init, rprintln, rtt_init, rtt_init_default, rtt_init_print,
};
use serde::{Deserialize, de};
use serde_cbor::de::from_mut_slice;

use panic_probe as _;

#[derive(Debug, Deserialize, Clone)]
enum IncomingMessage {
    Sensor(ServoSetup),
    Config(Config),
}

#[derive(Debug, Deserialize, Clone)]
struct ServoSetup {
    position_c0: u16,
    position_c1: u16,
    position_c2: u16,
    position_c3: u16,
    position_c4: u16,
    position_c5: u16,
    position_c6: u16,
    position_c7: u16,
    position_c8: u16,
    position_c9: u16,
    position_c10: u16,
    position_c11: u16,
}

#[derive(Debug, Deserialize, Clone)]
struct Config {
    position_speed: u16,
}

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

    spawner.spawn(reader(publisher, rx)).unwrap();
    // _col2.set_low();
    // _col3.set_low();

    spawner.spawn(servo_driver(sub, pwm));

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

fn smooth(current: f32, target: f32) -> f32 {
    let current_f = current as f32;
    let target_f = target as f32;

    let factor: f32 = 0.05;
    let result_f = ((1.0 - factor) * current_f) + (factor * target_f);

    result_f
}

#[embassy_executor::task]
async fn servo_driver(
    mut subcriber: Subscriber<'static, ThreadModeRawMutex, ServoSetup, 10, 1, 1>,
    mut pwm: Pca9685<Twim<'static, peripherals::TWISPI0>>,
) {
    let mut pwm_value = 300.0;
    loop {
        let new_setup = subcriber.next_message_pure().await;

        let val_for_pwm = new_setup.position_c0 as f32;

        while (val_for_pwm - pwm_value).abs() > 1.0 {
            pwm_value = smooth(pwm_value, val_for_pwm);
            pwm.set_channel_off(Channel::C15, pwm_value as u16).unwrap();
            pwm.set_channel_off(Channel::C14, pwm_value as u16).unwrap();

            Timer::after_millis(10).await;
        }
    }
}

#[embassy_executor::task]
async fn reader(
    mut publisher: Publisher<'static, ThreadModeRawMutex, ServoSetup, 10, 1, 1>,
    mut rx: UarteRx<'static, peripherals::UARTE0>,
) {
    let mut frame_buf = Vec::<u8, 256>::new();
    let mut decode_buf = [0u8; 256];

    loop {
        let mut byte = [0u8; 1];

        match rx.read(&mut byte).await {
            Ok(_) => {
                if byte[0] == 0x00 {
                    {
                        decode_buf[..frame_buf.len()].copy_from_slice(&frame_buf);

                        match decode_in_place(&mut decode_buf[..frame_buf.len()]) {
                            Ok(decoded_len) => {
                                let incomming = from_mut_slice::<IncomingMessage>(
                                    &mut decode_buf[..decoded_len],
                                )
                                .unwrap();

                                match incomming {
                                    IncomingMessage::Sensor(data) => {
                                        publisher.publish_immediate(data);
                                    }
                                    IncomingMessage::Config(config) => {}
                                }

                                frame_buf.clear();
                            }
                            Err(_cbor_err) => {}
                        }
                    }
                } else {
                    if frame_buf.push(byte[0]).is_err() {
                        frame_buf.clear();
                    }
                }
            }
            Err(_e) => {
                frame_buf.clear();
            }
        }
    }
}
