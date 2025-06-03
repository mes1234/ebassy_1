use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::Subscriber;

use crate::Config;

use rtt_target::rprintln;

#[embassy_executor::task]
pub async fn configuration_handler(
    mut subcriber: Subscriber<'static, ThreadModeRawMutex, Config, 10, 1, 1>,
    config: &'static Mutex<ThreadModeRawMutex, Config>,
) {
    loop {
        let new_config = subcriber.next_message_pure().await;

        let mut config_guard = config.lock().await;
        *config_guard = new_config;
        drop(config_guard);

        rprintln!("Configuration: New config saved")
    }
}
