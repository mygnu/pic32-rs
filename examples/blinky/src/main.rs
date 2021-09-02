//! Blinky
//!
//! The classical LED flashing example for a PIC32MX1xx or PIC32MX2xx in a
//! 28 pin package. The clock in generated by the internal RC oscillator. So,
//! no other external parts except the 10 µF capacitor at Vcap (pin 20) and a
//! LED connected via a resistor (e.g. 470 Ohms) to RB5 (pin 14) are needed.
//! RB0 (pin 4) is used to output some text via UART2.

#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::fmt::Write;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::*;
use mips_rt;
use mips_rt::entry;
use pic32_hal::clock::Osc;
use pic32_hal::coretimer::Delay;
use pic32_hal::gpio::GpioExt;
use pic32_hal::pac;
use pic32_hal::time::U32Ext;
use pic32_hal::uart::Uart;

// PIC32 configuration registers for PIC32MX1xx and PIC32MX2xx
#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb"
))]
#[link_section = ".configsfrs"]
#[no_mangle]
pub static CONFIGSFRS: [u32; 4] = [
    0x0fffffff, // DEVCFG3
    0xfff9ffd9, // DEVCFG2
    0xff7fcfd9, // DEVCFG1
    0x7ffffffb, // DEVCFG0
];

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pps = p.PPS;
    pps.rpb0r.write(|w| unsafe { w.rpb0r().bits(0b0010) }); // U2TX on RPB0

    // setup clock control object
    let sysclock = 40_000_000_u32.hz();
    let clock = Osc::new(p.OSC, sysclock);
    let mut timer = Delay::new(sysclock);

    let uart = Uart::uart2(p.UART2, &clock, 115200);
    timer.delay_ms(10u32);
    let (mut tx, _) = uart.split();
    writeln!(tx, "Blinky example").unwrap();

    let parts = p.PORTB.split();
    let mut led = parts.rb5.into_push_pull_output();

    let mut on = true;
    loop {
        writeln!(tx, "LED status: {}", on).unwrap();
        if on {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
        on = !on;
        timer.delay_ms(1000u32);
    }
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo<'_>) -> ! {
    loop {}
}
