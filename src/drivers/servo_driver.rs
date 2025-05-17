use embassy_nrf::peripherals;
use embassy_nrf::twim::Twim;
use embassy_time::Timer;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::pubsub::Subscriber;

use pwm_pca9685::{Channel, Pca9685};

use crate::ServoSetup;


#[embassy_executor::task]
pub async fn servo_driver(
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

fn smooth(current: f32, target: f32) -> f32 {
    let current_f = current as f32;
    let target_f = target as f32;

    let factor: f32 = 0.05;
    let result_f = ((1.0 - factor) * current_f) + (factor * target_f);

    result_f
}
