use crate::common::contracts::IncomingMessage;
use crate::common::contracts::ServoSetup;

use heapless::Vec;
use serde_cbor::de::from_mut_slice;
use cobs::decode_in_place;
use embassy_nrf::peripherals;
use embassy_nrf::uarte::UarteRx;

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
