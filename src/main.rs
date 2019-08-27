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

use termion::{async_stdin, clear, color, cursor, raw::IntoRawMode};

use std::time::Duration;

use std::thread;

use std::io::{stdout, Read, Write};

use std::env;

use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;
    let nm = String::from(&args[0].to_string());
    let mut sym = String::from("█");
    let mut fg_color = 1;
    let mut bg_color = 1;

    let x = 3;
    let y = 2;

    for i in 1..args.len() {
        if &args[i] == &"-c".to_string() {
            if args.len() <= i + 1 {
                println!("Invalid option for -C");
                help(&nm);
            } else {
                let ch = String::from(&args.get(i + 1).unwrap().to_string());
                let num = ch.parse::<u8>();
                match num {
                    Ok(val) => fg_color = val,
                    Err(e) => {
                        println!("Invalid option for -c: {}", e);
                        help(&nm);
                    }
                }
            }
        }
        if &args[i] == &"-C".to_string() {
            if args.len() <= i + 1 {
                println!("Invalid option for -C");
                help(&nm);
            } else {
                let ch = String::from(&args.get(i + 1).unwrap().to_string());
                let num = ch.parse::<u8>();
                match num {
                    Ok(val) => bg_color = val,
                    Err(e) => {
                        println!("Invalid option for -C: {}", e);
                        help(&nm);
                    }
                }
            }
        }
        if &args[i] == &"-d".to_string() {
            debug = true;
        }
        if &args[i] == &"-h".to_string() {
            help(&nm);
        }
        if &args[i] == &"-s".to_string() {
            if args.len() <= i + 1 {
                println!("Invalid option for -s");
                help(&nm);
            } else {
                let ch = args.get(i + 1).unwrap();
                sym = String::from(&ch.to_string());
            }
        }
    }
    let clock: &str = "%H:%M";
    let date: &str = "%F";
    let refresh = Duration::from_millis(100);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

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
            )
            .unwrap();
        }

        let time = Local::now().format(clock).to_string();
        let d_date = Local::now().format(date).to_string();
        let mut hour: Vec<[[bool; 6]; 5]> = Vec::new();
        for c in time.chars() {
            hour.push(symbol(c));
        }

        let mut pos_x = x;
        let mut pos_y = y;

        for digit in hour {
            for j in 0..digit.len() {
                for i in 0..digit[j].len() {
                    if digit[j][i] == true {
                        write!(
                            stdout,
                            "{}{}{}{}",
                            cursor::Goto(i as u16 + pos_x, j as u16 + pos_y),
                            color::Fg(color::AnsiValue(fg_color)),
                            color::Bg(color::AnsiValue(bg_color)),
                            sym
                        )
                        .unwrap();
                    }
                    write!(
                        stdout,
                        "{}{}{}",
                        cursor::Goto(i as u16 + pos_x, j as u16 + pos_y),
                        color::Fg(color::Reset),
                        color::Bg(color::Reset)
                    )
                    .unwrap();
                }
            }
            pos_x = pos_x + 7;
        }
        write!(stdout, "{}{}", cursor::Goto(12 + x, 6 + y), d_date).unwrap();
        stdout.flush().unwrap();

        let mut exit = 0;
        while time == Local::now().format(clock).to_string() {
            let ev = stdin.next();
            if let Some(Ok(b)) = ev {
                match b {
                    // Quit
                    b'q' => {
                        exit = 1;
                        break;
                    }

                    b'+' => {
                        fg_color = inc_u8(fg_color);
                        break;
                    }

                    b'-' => {
                        fg_color = dec_u8(fg_color);
                        break;
                    }

                    b'.' => {
                        bg_color = inc_u8(bg_color);
                        break;
                    }

                    b',' => {
                        bg_color = dec_u8(bg_color);
                        break;
                    }

                    _ => (),
                }
            }

            if resize_watcher(size) {
                break;
            }
            thread::sleep(refresh);
        }
        if exit == 1 {
            break;
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
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

fn inc_u8(mut val: u8) -> u8 {
    if val as i16 + 1 > 255 {
        val = 0;
    } else {
        val = val + 1;
    }
    val
}

fn dec_u8(mut val: u8) -> u8 {
    if val as i16 - 1 < 0 {
        val = 255;
    } else {
        val = val - 1;
    }
    val
}
