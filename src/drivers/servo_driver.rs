use embassy_nrf::peripherals;
use embassy_nrf::twim::Twim;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;

use pwm_pca9685::{Channel, Pca9685};

use rtt_target::rprintln;

use crate::Config;
use crate::common::contracts::ServoConfig;

// Represents current state of servos
static STATE: Mutex<ThreadModeRawMutex, [f32; 12]> = Mutex::new([0.0; 12]);

const POSITION_0: f32 = 100.0;
const POSITION_180: f32 = 600.0;

#[embassy_executor::task]
pub async fn servo_driver(
    mut pwm: Pca9685<Twim<'static, peripherals::TWISPI0>>,
    config: &'static Mutex<ThreadModeRawMutex, Config>,
    target: &'static Mutex<ThreadModeRawMutex, [f32; 12]>,
) {
    loop {
        // let new_setup = subcriber.next_message_pure().await;

        // Wait for next loop
        Timer::after_millis(5).await;

        // obtain configuration
        let config_guard = config.lock().await;
        let current_config = config_guard.clone();
        drop(config_guard); 

        // obtain position target
        let target_guard = target.lock().await; 
        let current_target = target_guard.clone();
        drop(target_guard);  

        // lock current state
        let mut state_guard = STATE.lock().await;

        for _ in 0..current_config.position_steps {
            for i in 0..12 {
                state_guard[i] = controll_servo(
                    state_guard[i],
                    current_target[i],
                    &mut pwm,
                    select_channel(i),
                    &current_config.c0_config,
                );
            }
        }
    }
}

fn controll_servo<'a>(
    state: f32,
    target: f32,
    pwm: &'a mut Pca9685<Twim<'static, peripherals::TWISPI0>>,
    channel: Channel,
    config: &ServoConfig,
) -> f32 {
    // Assume close enough but dont modify servo position
    if (state - target).abs() < 5.0 {
        return target;
    }

    let new_state = control(state, target);

    let pwm_setup = scale_and_cap(new_state, config);

    pwm.set_channel_off(channel, pwm_setup).unwrap();

    new_state
}

// Dummy implementation
fn control(current: f32, target: f32) -> f32 {
    let error = target - current;

    let result = current + error;

    result
}

fn select_channel(i: usize) -> Channel {
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
        _ => Channel::C15,
    };

    channel
}

fn scale_and_cap(state: f32, setup: &ServoConfig) -> u16 {
    let lower_bound = setup.lower as f32;
    let mut upper_bound = setup.higher as f32;

    if upper_bound > 180.0 {
        upper_bound = 180.0;
    }

    let clamped_angle = state.clamp(lower_bound, upper_bound) / (upper_bound - lower_bound);

    let result = POSITION_0 + (POSITION_180 - POSITION_0) * clamped_angle; 

    result as u16
}
