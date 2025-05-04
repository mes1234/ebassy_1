#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{AnyPin, Level, Output, OutputDrive, Pin},
    peripherals,
    twim::{self, Twim},
};
use embassy_time::Timer;
use pwm_pca9685::{Address, Channel, Pca9685};
use defmt;

use panic_probe as _;

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0  => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());


    let _row1 = led_pin(p.P0_21.degrade());
    let mut _col1 = led_pin(p.P0_28.degrade());
    let mut _col2 = led_pin(p.P0_11.degrade());
    let mut _col3 = led_pin(p.P0_31.degrade());
    
    let address = Address::default();

    defmt::info!("Yello");

    let config = twim::Config::default();

    let dev = Twim::new(p.TWISPI0, Irqs, p.P0_20.degrade(), p.P0_19.degrade(), config);

    let mut pwm =  Pca9685::new(dev, address).unwrap();

    _col1.set_low();
  
    match pwm.disable() {
        Ok(_) => defmt::info!("PWM disabled"),
        Err(e) => defmt::info!("Failed to disable PWM: {:?}", e),
    }

    _col2.set_low();

    pwm.set_prescale(121).unwrap();
     
    _col3.set_low();

    pwm.enable().unwrap();

    loop {
        pwm.set_channel_on_off(Channel::C15, 0, 2047).unwrap();
        Timer::after_millis(1000).await; 
    }
}

fn led_pin(pin: AnyPin) -> Output<'static> {
    Output::new(pin, Level::High, OutputDrive::Standard)
}
