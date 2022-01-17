#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cmsis_dsp_sys::{arm_sin_f32};
use cortex_m_rt::entry;
use hal::stm32;
use hal::gpio::AF1;
use panic_halt as _;
use stm32g4xx_hal as hal;

use hal::prelude::*;

#[entry]
fn main() -> ! {
    let sin = arm_sin_f32;
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let mut rcc = dp.RCC.constrain();
        let mut delay = cp.SYST.delay(&rcc.clocks);

        let gpioa = dp.GPIOA.split(&mut rcc);
        let led = gpioa.pa5.into_alternate::<AF1>();
        let mut pwm = dp.TIM2.pwm(led, 10000.hz(), &mut rcc);

        let mut now:f32 = 0.0;
        let a:f32 = (pwm.get_max_duty() / 2) as f32;
        pwm.enable();
        loop {
            unsafe {
                pwm.set_duty((a * (sin(now * 4.0) + 1.0)) as u32);
            };
            delay.delay_ms(10_u32);
            now += 0.01;
        }
    }
    loop {}
}