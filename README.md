<p align="center">
  <img alt="Logo" width="454" src="https://i.imgur.com/1TF28pq.png">
</p>

[![Rust](https://github.com/valebes/rsClock/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/valebes/rsClock/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

![Screenshot](https://i.imgur.com/CuirrjG.png)

## Introduction
rsClock is a simple terminal clock.

Now this project is active again.
I'll add new features in the future.

## Build
From Crates.io :

```console
unix@weeb:~$ cargo install rsclock
```

From GitHub :
```console
unix@weeb:~$ git clone https://github.com/valebes/rsClock
unix@weeb:~$ cd rsClock
unix@weeb:~$ cargo build --release
unix@weeb:~$ ./target/release/rsclock
```
## Usage
```console
unix@weeb:~$ usage : rsclock
    -s    Set custom symbol
    -S    Display seconds
    -T    Display only time
    -D    Display only date
    -f    Set foreground color [0-255] (Ansi value)
    -b    Set background color [0-255] (Ansi value)
    -u    Use 12-hour format (blocky AM/PM)
    -U    Use 12-hour format (dateline AM/PM)
    -d    Debug mode
    -c    Center the clock
    -v    Show rsClock version
    -h    Display this message
    -o    Print once, then exit
```
Use "+" and "-" to change foreground color.

Use "." and "," to change background color.

Use "q" to quit the program.

## Installation
[![Packaging status](https://repology.org/badge/vertical-allrepos/rsclock.svg)](https://repology.org/project/rsclock/versions)

