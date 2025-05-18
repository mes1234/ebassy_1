use crate::common::contracts::IncomingMessage;
use crate::common::contracts::ServoSetup;

use cobs::decode_in_place;
use embassy_nrf::uarte::UarteRx;
use embassy_nrf::uarte::UarteTx;
use heapless::Vec;
use serde_cbor::de::from_mut_slice;

use embassy_nrf::buffered_uarte::InterruptHandler;
use embassy_nrf::interrupt::typelevel;
use embassy_nrf::{
    bind_interrupts,
    gpio::{AnyPin, Level, Output, OutputDrive, Pin},
    peripherals::{self,UARTE0},
    twim::{self, Twim},
    uarte,
};

use rtt_target::rprintln;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::pubsub::Publisher;

#[embassy_executor::task]
pub async fn uart_reader_driver(
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

pub fn uart_init(
    timer_instance: peripherals::UARTE0,
    rx_pin: AnyPin,
    tx_pin: AnyPin,
    irqs: impl typelevel::Binding<
        <peripherals::UARTE0 as embassy_nrf::uarte::Instance>::Interrupt,
        uarte::InterruptHandler<peripherals::UARTE0>,
    > + 'static,
) -> (
    UarteTx<'static, peripherals::UARTE0>,
    UarteRx<'static, peripherals::UARTE0>,
) {
    let mut uart_config = uarte::Config::default();

    uart_config.parity = uarte::Parity::EXCLUDED;
    uart_config.baudrate = uarte::Baudrate::BAUD38400;

    let uart = uarte::Uarte::new(timer_instance, irqs, rx_pin, tx_pin, uart_config);

    let (mut tx, rx) = uart.split();

    rprintln!("System Booting: UART driver init: OK");
    (tx, rx)
}
