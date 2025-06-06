use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::Subscriber;

use crate::ServoSetup;

use rtt_target::rprintln;

#[embassy_executor::task]
pub async fn position_handler(
    mut subcriber: Subscriber<'static, ThreadModeRawMutex, ServoSetup, 10, 1, 1>,
    state: &'static Mutex<ThreadModeRawMutex, [f32; 12]>,
) {
    rprintln!("Configuration: Position reader bootstrap");

    loop {
        let new_state = subcriber.next_message_pure().await; 

        assign_target(new_state, &state).await; 
    }
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
