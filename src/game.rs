#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate compiler_builtins;
extern crate stm32f7_discovery as stm32f7; // initialization routines for .data and .bss
extern crate r0;



use alloc::Vec;
use graphics;
use stm32f7::{board, embedded, ethernet, lcd, sdram, system_clock, i2c, touch};


use super::WIDTH;
use super::HEIGHT;


const GRID_BLOCK_SIZE: usize = 10;

   

/**
 * Contains all necessary state information of a game.
 */
pub struct Game {
    graphics: graphics::Graphics,
    grid: Vec<Vec<Tile>>,
    i2c_3: stm32f7::i2c::I2C,
    touch: (),
}

/**
 * Possible tiles inside of the game grid.
 */
#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    SnakeHead,
    SnakeBody,
    SnakeTail,
    Apple,
}

impl Game {
    /**
     * Create a new game.
     */
    pub fn new(graphics: graphics::Graphics,i2c_3: stm32f7::i2c::I2C, touch: () ) -> Game {
        let game_width = WIDTH / GRID_BLOCK_SIZE;
        let game_height = HEIGHT / GRID_BLOCK_SIZE;
        let mut return_game = Game {
            graphics: graphics,
            grid: vec![vec![Tile::Empty; game_height]; game_width],
            i2c_3: i2c_3,
            touch: touch,
        };
        return_game.grid[25][10] = Tile::SnakeHead;
        return_game
    }

    /**
     * Draws current game state to screen.
     */
    pub fn draw_game(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                if self.grid[x][y] == Tile::SnakeHead {
                    self.graphics.print_square_size_color_at(
                        x * GRID_BLOCK_SIZE,
                        y * GRID_BLOCK_SIZE,
                        GRID_BLOCK_SIZE,
                        lcd::Color {
                            red: 255,
                            green: 0,
                            blue: 0,
                            alpha: 255,
                        },
                    );
                } else if self.grid[x][y] == Tile::SnakeBody {
                    self.graphics.print_square_size_color_at(
                        x * GRID_BLOCK_SIZE,
                        y * GRID_BLOCK_SIZE,
                        GRID_BLOCK_SIZE,
                        lcd::Color {
                            red: 255,
                            green: 0,
                            blue: 0,
                            alpha: 255,
                        },
                    );
                } else if self.grid[x][y] == Tile::SnakeTail {
                    self.graphics.print_square_size_color_at(
                        x * GRID_BLOCK_SIZE,
                        y * GRID_BLOCK_SIZE,
                        GRID_BLOCK_SIZE,
                        lcd::Color {
                            red: 255,
                            green: 0,
                            blue: 0,
                            alpha: 255,
                        },
                    );
                } else if self.grid[x][y] == Tile::Apple {
                    self.graphics.print_square_size_color_at(
                        x * GRID_BLOCK_SIZE,
                        y * GRID_BLOCK_SIZE,
                        GRID_BLOCK_SIZE,
                        lcd::Color {
                            red: 0,
                            green: 255,
                            blue: 0,
                            alpha: 255,
                        },
                    );
                }
                else if self.grid[x][y] == Tile::Empty {
                    self.graphics.print_square_size_color_at(
                        x * GRID_BLOCK_SIZE,
                        y * GRID_BLOCK_SIZE,
                        GRID_BLOCK_SIZE,
                        lcd::Color {
                            red: 0,
                            green: 0,
                            blue: 0,
                            alpha: 255,
                        },
                    );
                }
            }
        }
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    pub fn move_up(&mut self) {
        for x in 0..self.grid.len() - 1 {
            for y in 0..self.grid[0].len() - 1 {
                if self.grid[x][y] == Tile::SnakeHead {
                    self.grid[x][y - 1] = Tile::SnakeHead;
                    self.grid[x][y] = Tile::Empty;
                    return;
                }
            }
        }
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    pub fn move_down(&mut self) {
        for x in 0..self.grid.len() - 1 {
            for y in 0..self.grid[0].len() - 1 {
                if self.grid[x][y] == Tile::SnakeHead {
                    self.grid[x][y + 1] = Tile::SnakeHead;
                    self.grid[x][y] = Tile::Empty;
                    return;
                }
            }
        }
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    pub fn move_right(&mut self) {
        for x in 0..self.grid.len() - 1 {
            for y in 0..self.grid[0].len() - 1 {
                if self.grid[x][y] == Tile::SnakeHead {
                    self.grid[x + 1][y] = Tile::SnakeHead;
                    self.grid[x][y] = Tile::Empty;
                    return;
                }
            }
        }
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    pub fn move_left(&mut self) {
        for x in 0..self.grid.len() - 1 {
            for y in 0..self.grid[0].len() - 1 {
                if self.grid[x][y] == Tile::SnakeHead {
                    self.grid[x - 1][y] = Tile::SnakeHead;
                    self.grid[x][y] = Tile::Empty;
                    return;
                }
            }
        }
    }

    /**
     * Calls the correct function to turn to the right direction
     */

    // pub fn turn_position() {
    //     if 
    // }

    /**
     * Sets the direction chosen by the user
     */

    pub fn choose_direction(&mut self) {

            
            for touch in &self.touch::touches(&mut self.i2c_3).unwrap() {
                let mut x = touch.x;
                let mut y = touch.y;

            if x > 10 && x < 70 {
            self.move_down();   
            }
            else if x > 410 && x < 470 { 
            self.move_up();
            }

        
            }
    }
}
