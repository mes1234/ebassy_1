use embassy_nrf::{
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
    sda_pin: AnyPin,
    scl_pin: AnyPin,
    irqs: impl typelevel::Binding<
        <TWISPI0 as embassy_nrf::twim::Instance>::Interrupt,
        InterruptHandler<TWISPI0>,
    > + 'static,
) -> Pca9685<Twim<'static, peripherals::TWISPI0>> {
    let pca9685_address = Address::default();
    let twim_config = twim::Config::default();
    let twim_device = Twim::new(twim_instance, irqs, sda_pin, scl_pin, twim_config);
    rprintln!("System Booting: PWM driver init: Timer init OK");

    let mut pwm = match Pca9685::new(twim_device, pca9685_address) {
        Ok(pwm) => {
            rprintln!("System Booting: PWM driver init: PCA9685 init OK");
            pwm
        }
        Err(e) => {
            rprintln!("Error during PWM driver init:{:?}", e);
            panic!("Error during PWM driver init")
        }
    };

    match pwm.set_prescale(100) {
        Ok(_) => {
            rprintln!("System Booting: PWM driver init: PCA9685 prescale set OK");
        }
        Err(e) => {
            rprintln!(
                "System Booting: PWM driver init: Error during PCA9685 prescale set:{:?}",
                e
            );
            panic!("System Booting: PWM driver init: Error during PCA9685 prescale set")
        }
    };

    match pwm.enable() {
        Ok(_) => {
            rprintln!("System Booting: PWM driver init: PCA9685 enabled OK");
        }
        Err(e) => {
            rprintln!(
                "System Booting: PWM driver init: Error during PCA9685 enabled :{:?}",
                e
            );
            panic!("System Booting: PWM driver init: Error during PCA9685 enabled ")
        }
    }

    rprintln!("System Booting: PWM driver init: ALL OK");

    pwm
}
