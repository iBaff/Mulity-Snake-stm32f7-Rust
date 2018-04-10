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
extern crate arrayvec;
extern crate r0;
extern crate smoltcp;

#[macro_use]
use stm32f7::{board, embedded, ethernet, exceptions, lcd, sdram, system_clock, touch, i2c};

mod game;
mod graphics;
mod random;
mod network;

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
        rng,
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
    let lcd = lcd::init(ltdc, rcc, &mut gpio);
    let graphics = graphics::Graphics::new(lcd);

    //i2c
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    i2c_3.test_1();
    i2c_3.test_2();

    touch::check_family_id(&mut i2c_3).unwrap();

    /* ETHERNET START */
    let network;
    let mut ethernet_device = ethernet::EthernetDevice::new(
        Default::default(),
        Default::default(),
        rcc,
        syscfg,
        &mut gpio,
        ethernet_mac,
        ethernet_dma,
    );
    if let Err(e) = ethernet_device {
        println!("ethernet init failed: {:?}", e);
        panic!("ethernet init failed: {:?}", e);
    };
    if let Ok(ether) = ethernet_device {
        network = network::Network::new(ether, network::NetworkMode::client);
    }

    /* ETHERNET END */
    // l0et layer2 = lcd::Layer<lcd::FramebufferAl88>;

    let random_gen = random::Random::new(rng, rcc);
    // Initialize Game
    let mut game = game::Game::new(graphics, i2c_3, random_gen);
    gameloop(game);
}

fn gameloop(mut game: game::Game) -> ! {
    // Define Pictures
    let pic1: &[u8] = include_bytes!("../assets/Welcom_screen/Snake_base2.bmp");
    let pic2: &[u8] = include_bytes!("../assets/Welcom_screen/Snake_mouth_open.bmp");
    let pic3: &[u8] = include_bytes!("../assets/Welcom_screen/Snake_mouth_shut.bmp");
    game.graphics
        .print_bmp_at_with_rotaion(pic1, 0, 0, graphics::RotDirection::r_0);
    game.graphics.print_bmp_at_layer2(pic2, 300, 0);

    let welcome = "Welcome to Mulity-Snake! Touch Screen to start the Game\n";

    for c in welcome.chars() {
        if c == ' ' || c == '-' || c == '!' {
            print!("{}", c);
            system_clock::wait(10);
        } else {
            game.graphics.print_bmp_at_downwards(pic2, 188, 85);
            print!("{}", c);
            //system_clock::wait(10);
            game.graphics.print_bmp_at_downwards(pic3, 188, 85);
        }
    }
    // game.graphics.print_bmp_at_with_rotaion(pic, 0, 0, graphics::RotDirection::r_0);
    // game.graphics.print_bmp_at_with_rotaion(pic, 85, 0, graphics::RotDirection::r_90);
    // game.graphics.print_bmp_at_with_rotaion(pic, 170, 0, graphics::RotDirection::r_180);
    // game.graphics.print_bmp_at_with_rotaion(pic, 260, 0, graphics::RotDirection::r_270);
    /* Random Example */
    let ran = game.random_gen.random_range(0, 42);
    println!("A random number: 0 <= {} < 42!!!", ran);
    loop {
        // let ticks = system_clock::ticks();
        game.move_snake();
        game.snake_bite();
        game.draw_game();
        system_clock::wait(100);
    }
}
