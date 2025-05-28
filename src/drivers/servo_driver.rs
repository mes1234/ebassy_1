use embassy_nrf::peripherals;
use embassy_nrf::twim::Twim;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::Subscriber;
use embassy_time::Timer;

use pwm_pca9685::{Channel, Pca9685};

use rtt_target::{rprintln, rtt_init_print};

use crate::ServoSetup;

// Represents current state of servos
static STATE: Mutex<ThreadModeRawMutex, [f32; 12]> = Mutex::new([0.0; 12]);

// Represents target state of servos
static TARGET: Mutex<ThreadModeRawMutex, [f32; 12]> = Mutex::new([0.0; 12]);

#[embassy_executor::task]
pub async fn servo_driver(
    mut subcriber: Subscriber<'static, ThreadModeRawMutex, ServoSetup, 10, 1, 1>,
    mut pwm: Pca9685<Twim<'static, peripherals::TWISPI0>>,
) {
    let mut pwm_value = 300.0;
    loop {
        let new_setup = subcriber.next_message_pure().await;

        rprintln!("SERVO DRIVER: OBTAINED NEW VALUE");

        assign_target(new_setup, &TARGET).await;

        rprintln!("SERVO DRIVER: TARGETS INITALIZED");

        let mut target_guard = TARGET.lock().await;
        let mut state_guard = STATE.lock().await;

        for _ in 0..12 {
            for i in 0..12 {
                // Assume close enough but dont modify servo position
                if (state_guard[i] - target_guard[i]).abs() < 5.0 {
                    state_guard[i] = target_guard[i];
                    continue;
                }

                state_guard[i] = smooth(state_guard[i], target_guard[i]);

                match i {
                    0 => pwm
                        .set_channel_off(Channel::C0, state_guard[i] as u16)
                        .unwrap(),
                    1 => pwm
                        .set_channel_off(Channel::C1, state_guard[i] as u16)
                        .unwrap(),
                    2 => pwm
                        .set_channel_off(Channel::C2, state_guard[i] as u16)
                        .unwrap(),
                    3 => pwm
                        .set_channel_off(Channel::C3, state_guard[i] as u16)
                        .unwrap(),
                    4 => pwm
                        .set_channel_off(Channel::C4, state_guard[i] as u16)
                        .unwrap(),
                    5 => pwm
                        .set_channel_off(Channel::C5, state_guard[i] as u16)
                        .unwrap(),
                    6 => pwm
                        .set_channel_off(Channel::C6, state_guard[i] as u16)
                        .unwrap(),
                    7 => pwm
                        .set_channel_off(Channel::C7, state_guard[i] as u16)
                        .unwrap(),
                    8 => pwm
                        .set_channel_off(Channel::C8, state_guard[i] as u16)
                        .unwrap(),
                    9 => pwm
                        .set_channel_off(Channel::C9, state_guard[i] as u16)
                        .unwrap(),
                    10 => pwm
                        .set_channel_off(Channel::C10, state_guard[i] as u16)
                        .unwrap(),
                    11 => pwm
                        .set_channel_off(Channel::C11, state_guard[i] as u16)
                        .unwrap(),
                    _ => {}
                }
            }

            Timer::after_millis(5).await;
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

async fn assign_target(
    new_setup: ServoSetup,
    target: &'static Mutex<ThreadModeRawMutex, [f32; 12]>,
) {
    let mut target_guard = target.lock().await;

    target_guard[0] = new_setup.position_c0 as f32;
    target_guard[1] = new_setup.position_c1 as f32;
    target_guard[2] = new_setup.position_c2 as f32;
    target_guard[3] = new_setup.position_c3 as f32;
    target_guard[4] = new_setup.position_c4 as f32;
    target_guard[5] = new_setup.position_c5 as f32;
    target_guard[6] = new_setup.position_c6 as f32;
    target_guard[7] = new_setup.position_c7 as f32;
    target_guard[8] = new_setup.position_c8 as f32;
    target_guard[9] = new_setup.position_c9 as f32;
    target_guard[10] = new_setup.position_c10 as f32;
    target_guard[11] = new_setup.position_c11 as f32;
}
