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
 
    let config = twim::Config::default();

    let dev = Twim::new(p.TWISPI0, Irqs, p.P1_00.degrade(), p.P0_26.degrade(), config);

    let mut pwm =  Pca9685::new(dev, address).unwrap();

    _col1.set_low();

    _col2.set_low();

    pwm.set_prescale(100).unwrap();
     
    _col3.set_low();

    pwm.enable().unwrap();

    pwm.set_channel_on(Channel::All, 0).unwrap();

    let servo_min = 130; // minimum pulse length (out of 4096)
    let servo_max = 610; // maximum pulse length (out of 4096)
    let mut current = servo_min;
    let mut factor: i16 = 1;

    loop {
        pwm.set_channel_off(Channel::C15, current).unwrap();

        if current >= servo_max {
            factor = -5;
        }
        else if current < servo_min {
            factor = 5;
        }
        current = (current as i16 + factor) as u16;
        
        Timer::after_millis(5).await; 
    }
}

fn led_pin(pin: AnyPin) -> Output<'static> {
    Output::new(pin, Level::High, OutputDrive::Standard)
}
