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

use termion::{clear, color, cursor, raw::IntoRawMode};

use termion::event::{Key, Event, MouseEvent};

use termion::input::{TermRead, MouseTerminal};

use std::time::Duration;

use std::thread;

use std::io::{self, Write, stdout, stdin};

use std::env;

use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;
    let nm = String::from(&args[0].to_string());
    let mut sym = String::from("█");
    let mut fg_color = 1;
    let mut bg_color = 1;

    let mut stdout = stdout().into_raw_mode().unwrap();

    for i in 1..args.len() {
        if &args[i] == &"-d".to_string() {
            debug = true;
        }
        if &args[i] == &"-s".to_string() {
            if args.len() <= i + 1 {
                println!("Invalid option for -s");
                help(&nm);
            }
            else {
                let ch = args.get(i + 1).unwrap();
                sym = String::from(&ch.to_string());
            }
        }

        if &args[i] == &"-c".to_string() {
            if args.len() <= i + 1 {
                println!("Invalid option for --c");
                help(&nm);
            }
            else {
                let ch = String::from(&args.get(i + 1).unwrap().to_string());
                let num = ch.parse::<u8>();
                match num {
                    Ok(val) => fg_color = val,
                    Err(e) => { println!("Invalid option for -c: {}", e); help(&nm); },
                }
            }
        }

        if &args[i] == &"-C".to_string() {
            if args.len() <= i + 1 {
                println!("Invalid option for --c");
                help(&nm);
            }
            else {
                let ch = String::from(&args.get(i + 1).unwrap().to_string());
                let num = ch.parse::<u8>();
                match num {
                    Ok(val) => bg_color = val,
                    Err(e) => { println!("Invalid option for -c: {}", e); help(&nm); },
                }
            }
        }
    }
    let clock: &str = "%H:%M";
    let date: &str = "%F";
    let refresh = Duration::from_millis(100);
    
    loop {
        let size = termion::terminal_size().unwrap();
        write!(stdout, "\n{}{}\n", cursor::Hide, clear::All).unwrap();

        // terminal size is only for debug
        if debug {
            write!(
                stdout,
                "{}(x:{},y:{})",
                cursor::Goto(size.0 - 12, 1),
                size.0,
                size.1
            ).unwrap();
        }

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
                        write!(
                            stdout,
                            "{}{}{}{}",
                            cursor::Goto(i as u16 + x, j as u16 + y),
                            color::Fg(color::AnsiValue(fg_color)),
                            color::Bg(color::AnsiValue(bg_color)),
                            sym
                        ).unwrap();
                    }
                    write!(
                        stdout,
                        "{}{}{}",
                        cursor::Goto(i as u16 + x, j as u16 + y),
                        color::Fg(color::Reset),
                        color::Bg(color::Reset)
                    ).unwrap();
                }
            }
            x = x + 7;
        }
        write!(stdout, "{}{}", cursor::Goto(13, 6 + y), d_date).unwrap();
        io::stdout().flush().unwrap();
        while time == Local::now().format(clock).to_string() {
            let stdin = stdin();
            for c in stdin.events() {
                let evt = c.unwrap();
                match evt {
                    Event::Key(Key::Char('q')) => process::exit(1),
                    _ => (),
                }
            }
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

fn help(nm: &String) {
    println!("usage : {}", nm);
    println!("    -s    Set custom symbol");
    println!("    -c    Set foreground color [0-255] (Ansi value)");
    println!("    -C    Set background color [0-255] (Ansi value)");
    println!("    -d    Debug mode");
    println!("    -h    Display this message");
    process::exit(1);
}
