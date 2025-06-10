#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use Level::{High, Low};
use ch32_hal as hal;
use embassy_executor::Spawner;
use hal::delay::Delay;
use hal::exti::ExtiInput;
use hal::gpio::{Level, Output, Pull};

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI;
    let p = hal::init(config);

    let mut read_vcc_en = Output::new(p.PC4, Low, Default::default());
    let mut sel_led = Output::new(p.PC5, High, Default::default());
    let mut mux_sel = Output::new(p.PC6, Low, Default::default());
    let sel_btn = ExtiInput::new(p.PC7, p.EXTI7, Pull::None);

    loop {
        if sel_btn.is_high() {
            read_vcc_en.set_level(Low);
            sel_led.set_level(High);
            mux_sel.set_level(Low);
        } else {
            read_vcc_en.set_level(High);
            sel_led.set_level(Low);
            mux_sel.set_level(High);
        }

        Delay.delay_ms(100);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = hal::println!("\n\n\n{}", info);

    loop {}
}
