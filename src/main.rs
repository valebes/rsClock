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

const VERSION: &str = "0.1.10";

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

const DASH: [[bool; 6]; 5] = [
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
    [false, true, true, true, true, false],
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
];

const ERR: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
];

const SPACE: [[bool; 6]; 5] = [
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
];

const A: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, false, false, true, true],
];

const P: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, false, false, false, false],
];

const M: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, true, false, true],
    [true, true, false, true, false, true],
    [true, true, false, true, false, true],
    [true, true, false, true, false, true],
];

extern crate chrono;
extern crate termion;

use chrono::prelude::*;

use termion::{async_stdin, clear, color, cursor, raw::IntoRawMode, raw::RawTerminal};

use std::time::Duration;

use std::thread;

use std::io::{stdout, Read, Write};

use std::env;

use std::process;

/* Functions */

fn resize_watcher<W: Write>(size: (u16, u16), stdout: &mut RawTerminal<W>) -> bool {
    if size != termion::terminal_size().unwrap() {
        write!(stdout, "{}", clear::All).unwrap();
        true
    } else {
        false
    }
}

fn center(x_mod: u16, y_mod: u16, x_size: u16, y_size: u16) -> (u16, u16) {
    let size = termion::terminal_size().unwrap();
    let mut x = 1;
    let mut y = 1;
    if (size.0 as i32) / 2 - (x_size as i32) / 2 + x_mod as i32 > 0 {
        x = (size.0) / 2 - (x_size / 2) + x_mod;
    }
    if (size.1 as i32) / 2 - (y_size as i32) / 2 + y_mod as i32 > 0 {
        y = (size.1) / 2 - (y_size / 2) + y_mod;
    }
    (x, y)
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
        '-' => DASH,
        ' ' => SPACE,
        'A' => A,
        'P' => P,
        'M' => M,
        _ => ERR,
    }
}

fn help(nm: &String) {
    println!("usage : {}", nm);
    println!("    -s    Set custom symbol");
    println!("    -S    Display seconds");
    println!("    -T    Display only time");
    println!("    -D    Display only date");
    println!("    -f    Set foreground color [0-255] (Ansi value)");
    println!("    -b    Set background color [0-255] (Ansi value)");
    println!("    -u    Use 12-hour format (blocky AM/PM)");
    println!("    -U    Use 12-hour format (dateline AM/PM)");
    println!("    -d    Debug mode");
    println!("    -c    Center the clock");
    println!("    -v    Show rsClock version");
    println!("    -h    Display this message");
    process::exit(1);
}

fn draw<W: Write>(
    hour: Vec<[[bool; 6]; 5]>,
    sym: String,
    mut pos_x: u16,
    pos_y: u16,
    fg_color: u8,
    bg_color: u8,
    stdout: &mut RawTerminal<W>,
) {
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
                } else {
                    write!(
                        stdout,
                        "{}{}{}{}",
                        cursor::Goto(i as u16 + pos_x, j as u16 + pos_y),
                        color::Fg(color::Reset),
                        color::Bg(color::Reset),
                        " "
                    )
                    .unwrap();
                }
            }
        }
        pos_x += 7;
    }
}

/* Main */

fn main() {
    let args: Vec<String> = env::args().collect();
    /* Set default values */
    let mut debug = false; // Debug mode
    let nm = String::from(&args[0].to_string());
    let mut sym = String::from("█"); // Symbol
    let mut fg_color = 1; // Fg color
    let mut bg_color = 1; // Fg color
    let mut center_clock = false; // Center clock (Default: no)
    let mut seconds = false; // Display seconds (Default: no)
    let mut time_only = false;
    let mut date_only = false;
    let mut twelve_hour_block = false;
    let mut twelve_hour_line = false;
    let mut only_once = false;

    /* Default position modifier */
    let x_mod = 1;
    let y_mod = 1;

    /* Default date size */
    let mut x_size = 34;
    let y_size = 7;

    /* Args parsing */
    for i in 1..args.len() {
        if &args[i] == &"-f".to_string() {
            // fg_color
            if args.len() <= i + 1 {
                println!("Invalid option for -f");
                help(&nm);
            } else {
                let ch = String::from(&args.get(i + 1).unwrap().to_string());
                let num = ch.parse::<u8>();
                match num {
                    Ok(val) => fg_color = val,
                    Err(e) => {
                        println!("Invalid option for -f: {}", e);
                        help(&nm);
                    }
                }
            }
        }
        if &args[i] == &"-b".to_string() {
            // bg_color
            if args.len() <= i + 1 {
                println!("Invalid option for -b");
                help(&nm);
            } else {
                let ch = String::from(&args.get(i + 1).unwrap().to_string());
                let num = ch.parse::<u8>();
                match num {
                    Ok(val) => bg_color = val,
                    Err(e) => {
                        println!("Invalid option for -b: {}", e);
                        help(&nm);
                    }
                }
            }
        }
        if &args[i] == &"-d".to_string() {
            // Debug mode
            debug = true;
        }
        if &args[i] == &"-h".to_string() {
            // Help
            help(&nm);
        }
        if &args[i] == &"-s".to_string() {
            // Custom symbol
            if args.len() <= i + 1 {
                println!("Invalid option for -s");
                help(&nm);
            } else {
                let ch = args.get(i + 1).unwrap();
                sym = String::from(&ch.to_string());
            }
        }
        if &args[i] == &"-S".to_string() {
            // Display seconds
            seconds = true;
        }
        if &args[i] == &"-v".to_string() {
            // Priny rsClock version
            println!("rsClock {}", VERSION);
            process::exit(1);
        }
        if &args[i] == &"-c".to_string() {
            center_clock = true;
        }
        if &args[i] == &"-u".to_string() {
            twelve_hour_line = true;
        }
        if &args[i] == &"-U".to_string() {
            twelve_hour_block = true;
        }
        if &args[i] == &"-T".to_string() {
            time_only = true;
        }
        if &args[i] == &"-D".to_string() {
            date_only = true;
        }
        if &args[i] == &"-o".to_string() {
            only_once = true;
        }
    }

    /* Setting format */
    let mut format = if twelve_hour_block || twelve_hour_line {
        "%I:%M".to_string()
    } else {
        "%H:%M".to_string()
    };

    if seconds {
        format += ":%S";
        x_size += 21;
    }

    if twelve_hour_block {
        format += " %p";
        x_size += 21;
    }

    let clock: &str = format.as_str();
    let date: &str = if twelve_hour_line { "%F %p" } else { "%F" };

    /* Setting refresh value */
    let refresh = Duration::from_millis(100);

    /* Prepare stdout and stdin */
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();
    let mut size = termion::terminal_size().unwrap();

    let mut x = 2;
    let mut y = 2;

    if center_clock {
        let pos = center(x_mod, y_mod, x_size, y_size);
        x = pos.0;
        y = pos.1;
    }

    write!(stdout, "\n{}{}\n", cursor::Hide, clear::All).unwrap();
    /* Start loop */
    loop {
        // Display terminal size only in debug mode
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

        let time = Local::now().format(clock).to_string(); // get time
        let d_date = Local::now().format(date).to_string(); // get date
        let mut hour: Vec<[[bool; 6]; 5]> = Vec::new();

        if !date_only {
            for c in time.chars() {
                hour.push(symbol(c));
            }
        } else {
            for c in d_date.chars() {
                hour.push(symbol(c));
            }
        }

        /* Draw time and print date */
        draw(hour, sym.clone(), x, y, fg_color, bg_color, &mut stdout);

        if !time_only && !date_only {
            write!(
                stdout,
                "{}{}{}{}",
                cursor::Goto((x_size / 2) - (d_date.len() as u16) / 2 + x, 6 + y),
                color::Fg(color::Reset),
                color::Bg(color::Reset),
                d_date,
            )
            .unwrap();
        } else {
            write!(
                stdout,
                "{}{}{}",
                cursor::Goto((x_size / 2) - (d_date.len() as u16) / 2 + x, 6 + y),
                color::Fg(color::Reset),
                color::Bg(color::Reset),
            )
            .unwrap();
        }

        stdout.flush().unwrap();
        if only_once {
            break;
        }

        /* Wait for the next cycle */
        let mut exit = 0;
        while time == Local::now().format(clock).to_string() {
            let ev = stdin.next(); // Get user input
            if let Some(Ok(b)) = ev {
                match b {
                    b'q' => {
                        // Exit
                        exit = 1;
                        break;
                    }

                    b'+' => {
                        // Change fg +
                        fg_color = fg_color.wrapping_add(1);
                        break;
                    }

                    b'-' => {
                        // Change fg -
                        fg_color = fg_color.wrapping_sub(1);
                        break;
                    }

                    b'.' => {
                        // Change bg +
                        bg_color = bg_color.wrapping_add(1);
                        break;
                    }

                    b',' => {
                        // Change bg -
                        bg_color = bg_color.wrapping_sub(1);
                        break;
                    }

                    _ => (),
                }
            }

            /* Watch terminal size */
            if resize_watcher(size, &mut stdout) {
                if center_clock {
                    let new_size = center(x_mod, y_mod, x_size, y_size);
                    x = new_size.0;
                    y = new_size.1;
                }
                size = termion::terminal_size().unwrap();
                break; // -> Re-draw
            }
            thread::sleep(refresh); // Sleep
        }

        if exit == 1 {
            // Quit
            break;
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap(); // Reset cursor
}
