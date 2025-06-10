#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

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

    let mut read_vcc_en = Output::new(p.PC4, Level::Low, Default::default());
    let mut sel_led = Output::new(p.PC5, Level::High, Default::default());
    let mut mux_sel = Output::new(p.PC6, Level::Low, Default::default());
    let mut sel_btn = ExtiInput::new(p.PC7, p.EXTI7, Pull::None);

    loop {
        sel_btn.wait_for_falling_edge().await;
        read_vcc_en.toggle();
        sel_led.toggle();
        mux_sel.toggle();
        Delay.delay_ms(300);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = hal::println!("\n\n\n{}", info);

    loop {}
}
