use embassy_nrf::peripherals;
use embassy_nrf::twim::Twim;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::Subscriber;
use embassy_time::Timer;

use pwm_pca9685::{Channel, Pca9685};

use rtt_target::rprintln;

use crate::common::contracts::ServoConfig;
use crate::Config;
use crate::ServoSetup;

// Represents current state of servos
static STATE: Mutex<ThreadModeRawMutex, [f32; 12]> = Mutex::new([0.0; 12]);

// Represents target state of servos
static TARGET: Mutex<ThreadModeRawMutex, [f32; 12]> = Mutex::new([0.0; 12]);

const POSITION_0: f32 = 100.0;
const POSITION_180: f32 = 600.0;

#[embassy_executor::task]
pub async fn servo_driver(
    mut subcriber: Subscriber<'static, ThreadModeRawMutex, ServoSetup, 10, 1, 1>,
    mut pwm: Pca9685<Twim<'static, peripherals::TWISPI0>>,
    config: &'static Mutex<ThreadModeRawMutex, Config>,
) {
    loop {
        let new_setup = subcriber.next_message_pure().await;

        let config_guard = config.lock().await;

        let cuurent_config = config_guard.clone();

        drop(config_guard);

        //rprintln!("SERVO DRIVER: OBTAINED NEW VALUE");

        assign_target(new_setup, &TARGET).await;

        //rprintln!("SERVO DRIVER: TARGETS INITALIZED");

        let target_guard = TARGET.lock().await;
        let mut state_guard = STATE.lock().await;

        rprintln!("SERVO DRIVER: RUN STEPS {}", cuurent_config.position_steps);

        for _ in 0..cuurent_config.position_steps {
            for i in 0..12 {
                // Assume close enough but dont modify servo position
                if (state_guard[i] - target_guard[i]).abs() < 5.0 {
                    state_guard[i] = target_guard[i];
                    continue;
                }

                state_guard[i] = smooth(state_guard[i], target_guard[i]);

                let channel = match i {
                    0 => Channel::C0,
                    1 => Channel::C1,
                    2 => Channel::C2,
                    3 => Channel::C3,
                    4 => Channel::C4,
                    5 => Channel::C5,
                    6 => Channel::C6,
                    7 => Channel::C7,
                    8 => Channel::C8,
                    9 => Channel::C9,
                    10 => Channel::C10,
                    11 => Channel::C11,
                    _ => Channel::C15
                };

                let pwm_setup = scale_and_cap(state_guard[i],&cuurent_config.c0_config);

                pwm.set_channel_off(channel, pwm_setup).unwrap();
            }

            Timer::after_millis(5).await;
        }
    }
}

// TODO this function doesnt work!!!!
fn smooth(current: f32, target: f32) -> f32 {
    let current_f = current as f32;
    let target_f = target as f32;

    let factor: f32 = 0.05;
    let result_f = ((1.0 - factor) * current_f) + (factor * target_f);

    // result_f

    target
}

fn scale_and_cap(state: f32, setup: &ServoConfig) -> u16 {
    
    let lower_bound = setup.lower as f32;
    let mut upper_bound = setup.higher as f32;

    if upper_bound > 180.0 {
        upper_bound = 180.0;
    }

    let clamped_angle = state.clamp(lower_bound, upper_bound)/(upper_bound - lower_bound);


    let result = POSITION_0 + (POSITION_180 - POSITION_0) * clamped_angle;

    rprintln!("SERVO DRIVER: NEW_POSITION {}", result);

    result as u16
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
