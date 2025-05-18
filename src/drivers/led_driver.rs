use embassy_nrf::gpio::OutputDrive;
use embassy_nrf::gpio::Level;
use embassy_nrf::gpio::Output;
use embassy_nrf::gpio::AnyPin;

pub fn led_pin(pin: AnyPin) -> Output<'static> {
    Output::new(pin, Level::High, OutputDrive::Standard)
}
