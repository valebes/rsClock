/*
                               
██████╗ ███████╗ ██████╗██╗      ██████╗  ██████╗██╗  ██╗
██╔══██╗██╔════╝██╔════╝██║     ██╔═══██╗██╔════╝██║ ██╔╝
██████╔╝███████╗██║     ██║     ██║   ██║██║     █████╔╝ 
██╔══██╗╚════██║██║     ██║     ██║   ██║██║     ██╔═██╗ 
██║  ██║███████║╚██████╗███████╗╚██████╔╝╚██████╗██║  ██╗
╚═╝  ╚═╝╚══════╝ ╚═════╝╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝

   Copyright (c) 2019 Valerio Besozzi
   MIT License
   https://github.com/valebes/rsClock
              
*/

extern crate chrono;
extern crate termion;

const ONE: [[bool; 6]; 5] = [
    [false, false, true, true, false, false],
    [false, false, true, true, false, false],
    [false, false, true, true, false, false],
    [false, false, true, true, false, false],
    [false, false, true, true, false, false],
];

const TWO: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
];

const THREE: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
];

const FOUR: [[bool; 6]; 5] = [
    [true, true, false, false, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [false, false, false, false, true, true],
];

const FIVE: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
];

const SIX: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
];

const SEVEN: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [false, false, false, false, true, true],
    [false, false, false, false, true, true],
    [false, false, false, false, true, true],
];

const EIGHT: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
];

const NINE: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
];

const ZERO: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, false, false, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
];

const DIV: [[bool; 6]; 5] = [
    [false, false, false, false, false, false],
    [false, false, true, true, false, false],
    [false, false, false, false, false, false],
    [false, false, true, true, false, false],
    [false, false, false, false, false, false],
];

use chrono::prelude::*;

use termion::{clear, color::*, cursor};

use std::time::{Duration, Instant};

use std::thread;

use std::io::{self, Write};

fn main() {
    let clock: &str = "%H:%M";
    let date: &str = "%F";
    let refresh = Duration::from_millis(100);

    loop {
        let size = termion::terminal_size().unwrap();
        println!("\n{}{}", cursor::Hide, clear::All);
        let time = Local::now().format(clock).to_string();
        let d_date = Local::now().format(date).to_string();
        let mut hour: Vec<[[bool; 6]; 5]> = Vec::new();
        for c in time.chars() {
            hour.push(symbol(c));
        }
        let mut x = 1;
        let mut y = 2;
        for digit in hour {
            for j in 0..digit.len() {
                for i in 0..digit[j].len() {
                    if digit[j][i] == true {
                        print! {"{}█", cursor::Goto(i as u16 +x, j as u16 +y)};
                    }
                }
            }
            x = x + 7;
        }
        print!("{}{}", cursor::Goto(13, 6 + y), d_date);
        io::stdout().flush().unwrap();
        while time == Local::now().format(clock).to_string() {
            if resize_watcher(size) {
                break;
            }
            thread::sleep(refresh);
        }
    }
}

fn resize_watcher(size: (u16, u16)) -> bool {
    if size != termion::terminal_size().unwrap() {
        println!("{}", clear::All);
        true
    } else {
        false
    }
}

fn symbol(ch: char) -> [[bool; 6]; 5] {
    match ch {
        '1' => ONE,
        '2' => TWO,
        '3' => THREE,
        '4' => FOUR,
        '5' => FIVE,
        '6' => SIX,
        '7' => SEVEN,
        '8' => EIGHT,
        '9' => NINE,
        '0' => ZERO,
        ':' => DIV,
        _ => ZERO,
    }
}
