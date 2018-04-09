#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![feature(alloc)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate compiler_builtins;
#[macro_use]
extern crate stm32f7_discovery as stm32f7; // initialization routines for .data and .bss

// initialization routines for .data and .bss
#[macro_use]
extern crate alloc;
extern crate r0;
extern crate smoltcp;

use stm32f7::{board, embedded, ethernet, lcd, sdram, system_clock, i2c, touch};
use stm32f7::ethernet::IP_ADDR;
use smoltcp::socket::{Socket, SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::wire::{IpAddress, IpEndpoint};
use smoltcp::time::Instant;
use alloc::Vec;

mod graphics;
mod game;

pub const HEIGHT: usize = 272;
pub const WIDTH: usize = 480;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static mut __DATA_END: u32;
        static mut __DATA_START: u32;

        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    // initializes the .data section (copy the data segment initializers from flash to RAM)
    r0::init_data(&mut __DATA_START, &mut __DATA_END, &__DATA_LOAD);
    // zeroes the .bss section
    r0::zero_bss(&mut __BSS_START, &__BSS_END);

    stm32f7::heap::init();

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);
    asm!("DSB; ISB;"::::"volatile"); // pipeline flush

    main(board::hw());
}

fn main(hw: board::Hardware) -> ! {
    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        syscfg,
        ethernet_mac,
        ethernet_dma,
        i2c_3,
        ..
    } = hw;

    use embedded::interfaces::gpio::{self, Gpio};
    let mut gpio = Gpio::new(
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
    );

    system_clock::init(rcc, pwr, flash);
    // enable all gpio ports
    rcc.ahb1enr.update(|r| {
        r.set_gpioaen(true);
        r.set_gpioben(true);
        r.set_gpiocen(true);
        r.set_gpioden(true);
        r.set_gpioeen(true);
        r.set_gpiofen(true);
        r.set_gpiogen(true);
        r.set_gpiohen(true);
        r.set_gpioien(true);
        r.set_gpiojen(true);
        r.set_gpioken(true);
    });

    // init sdram (needed for display buffer)
    sdram::init(rcc, fmc, &mut gpio);
    // lcd controller
    // let ltdc_pointer = ltdc as *mut board::ltdc::Ltdc;
    let lcd = lcd::init(ltdc, rcc, &mut gpio);
    let graphics = graphics::Graphics::new(lcd);

    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    let mut touch = touch::check_family_id(&mut i2c_3).unwrap();


    // unsafe {
    //     (*ltdc_pointer).l1cacr.update(|r| r.set_consta(255));
    //     (*ltdc_pointer).l2cacr.update(|r| r.set_consta(255));
    // }

    /* ETHERNET START */
    
    /* ETHERNET END */

    gameloop(graphics,i2c_3,touch);
}

fn gameloop(mut graphics: graphics::Graphics,mut i2c_3: stm32f7::i2c::I2C,mut touch: () ) -> ! {
    // Define Colors
    let red = lcd::Color {red:255, green:0, blue:0, alpha: 255};
    let green = lcd::Color {red:0, green:255, blue:0, alpha: 255};
    let blue = lcd::Color {red:0, green:0, blue:255, alpha: 255};
    // For iterating colors
    let colors = [red, green, blue];
    let mut chosen_color = 0; // colors[chosen_color];
    // Coordinates to draw to
    let mut x = 0;
    let mut y = 0;
    // Initialize Game
    let mut game = game::Game::new(graphics,i2c_3,touch);
    loop {
        // let ticks = system_clock::ticks();
        game.draw_game();
        game.move_right();
        system_clock::wait(1000);
    }
}
