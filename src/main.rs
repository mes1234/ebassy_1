#![no_std]
#![no_main]
 
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_nrf::{
    bind_interrupts,
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},  
}; 
use panic_probe as _; 

#[embassy_executor::main]
async fn main(_spawner: Spawner) { 
    let p = embassy_nrf::init(Default::default());

    let mut button_a = Input::new(p.P0_14, Pull::None);
    let mut button_b = Input::new(p.P0_23, Pull::None);

    let _row1 = led_pin(p.P0_21.degrade());
    let mut _col1 = led_pin(p.P0_28.degrade());
    let mut _col2 = led_pin(p.P0_11.degrade());

    _spawner.spawn(blinker_task(_col2)).unwrap();

    loop {
        button_a.wait_for_low().await;
        _col1.set_high(); 
        button_a.wait_for_high().await;
        _col1.set_low();
        
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn blinker_task(mut led : Output<'static>){
    loop { 
        Timer::after_millis(1000).await;
        led.set_low();
        Timer::after_millis(1000).await;
        led.set_high();
        
    } 
}

fn led_pin(pin: AnyPin) -> Output<'static> {
    Output::new(pin, Level::High, OutputDrive::Standard)
}