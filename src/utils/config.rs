use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::pubsub::Subscriber;
use embassy_sync::mutex::Mutex;

use crate::Config;

use rtt_target::{rprintln, rtt_init_print};

#[embassy_executor::task]
pub async fn configuration_handler(
    mut subcriber: Subscriber<'static, ThreadModeRawMutex, Config, 10, 1, 1>,
    config: &'static Mutex<ThreadModeRawMutex, Config>
) {
    loop {
        let new_config = subcriber.next_message_pure().await;

        let mut config_guard = config.lock().await;
        *config_guard = new_config;
        drop(config_guard);

        rprintln!("New configuration obtained and saved")
    }
}
