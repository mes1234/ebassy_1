use embassy_nrf::{
    Peripherals, bind_interrupts,
    gpio::AnyPin,
    interrupt::typelevel,
    peripherals::{self, TWISPI0},
    twim::InterruptHandler,
    twim::{self, Twim},
};
use pwm_pca9685::{Address, Pca9685};
use rtt_target::rprintln;

pub fn pwm_init(
    twim_instance: peripherals::TWISPI0,
    scl_pin: AnyPin,
    sda_pin: AnyPin,
    irqs: impl typelevel::Binding<
        <TWISPI0 as embassy_nrf::twim::Instance>::Interrupt,
        InterruptHandler<TWISPI0>,
    > + 'static,
) -> Pca9685<Twim<'static, peripherals::TWISPI0>> {
    let pca9685_address = Address::default();

    rprintln!("System Booting: PWM driver init: Configuration OK");

    let twim_config = twim::Config::default();

    rprintln!("System Booting: PWM driver init: Timer config OK");

    let twim_device = Twim::new(twim_instance, irqs, sda_pin, scl_pin, twim_config);

    rprintln!("System Booting: PWM driver init: Timer init OK");

    let mut pwm = Pca9685::new(twim_device, pca9685_address).unwrap();

    rprintln!("System Booting: PWM driver init: PCA9685 init OK");

    pwm.set_prescale(100).unwrap();

    rprintln!("System Booting: PWM driver init: PCA9685 prescale set OK");

    pwm.enable().unwrap();

    rprintln!("System Booting: PWM driver init: PCA9685 enabled OK");

    rprintln!("System Booting: PWM driver init: ALL OK");

    pwm
}
