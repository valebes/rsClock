<p align="center">
  <img alt="Logo" width="454" src="https://i.imgur.com/1TF28pq.png">
</p>

[![Build Status](https://travis-ci.org/valebes/rsClock.svg?branch=master)](https://travis-ci.org/valebes/rsClock)
![GitHub](https://img.shields.io/github/license/valebes/rsClock.svg)
[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

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
  		 -f    Set foreground color [0-255] (Ansi value)
   		 -b    Set background color [0-255] (Ansi value)
 		 -d    Debug mode
  		 -c    Center the clock
    		 -v    Show rsClock version
   		 -h    Display this message
```
Use "+" and "-" to change foreground color.

Use "." and "," to change background color.

Use "q" to quit the program.

## To-Do
* [ ] Complete code refactoring (working on it)
* [x] Colors support 
* [ ] Alarms
* [ ] Config file
* [ ] Themes support
* [ ] Documentation
#### Work in progress
