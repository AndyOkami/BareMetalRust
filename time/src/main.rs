/*!
 * A basic implementation of the `millis()` function from Arduino:
 *
 *     https://www.arduino.cc/reference/en/language/functions/time/millis/
 *
 * Uses timer TC0 and one of its interrupts to update a global millisecond
 * counter.  A walkthough of this code is available here:
 *
 *     https://blog.rahix.de/005-avr-hal-millis/
 */

 /*
 * A millis interpretation for arduino mega2560. 
 * Program counts how long program has been running
 * turns off led for power saving
 * and turns them on when recieving usart communication.
 * Tells how long the board has been on in milliseconds after recieving serial data.
 * 
 * Creator: Antti Kumavaara
 * Student number: 265844
 * emai: antti.kumavaara@tuni.fi
 * */ 

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::prelude::*;
use core::cell;
use panic_halt as _;

use embedded_hal::{serial::Read};


// Implementation for milis function of ardunio made using rust
// Used uno implementation as a base.
//------------------------------------------------
// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u32 = 125;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

// Configure the timer for the above interval (in CTC mode)
// and enable its interrupt.
fn millis_init(tc0: arduino_hal::pac::TC0) {
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    // Enable output compare match interrupt
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega2560)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

// ----------------------------------------------------------------------------

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    
    millis_init(dp.TC0);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    // Led for detecting sending data
    // On while getting data, off when power safe mode.
    let mut led = pins.d13.into_output().downgrade();

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    // Wait for a character and print current time once it is received
    loop {
        let b = nb::block!(serial.read()).void_unwrap();
        // turn off powersave mode after reading data
        dp.CPU.smcr.write(|w| unsafe{w.bits(0b0110)});
        led.set_high();
        let time = millis();
        ufmt::uwriteln!(&mut serial, "Got {} after {} ms!\r", b, time).void_unwrap();
        arduino_hal::delay_ms(1000);
        led.set_low();
        
        // turn on powersave mode after reading data
        dp.CPU.smcr.write(|w| unsafe{w.bits(0b0111)});
    
    }
}