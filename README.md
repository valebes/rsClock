<p align="center">
  <img alt="Logo" width="454" src="https://i.imgur.com/1TF28pq.png">
</p>

[![Build Status](https://travis-ci.org/valebes/rsClock.svg?branch=master)](https://travis-ci.org/valebes/rsClock)
![GitHub](https://img.shields.io/github/license/valebes/rsClock.svg)

![Screenshot](https://i.imgur.com/oyCXhXU.png)

## Introduction
rsClock is a simple terminal clock.

Now is only a simple demo, I'll add some features in the future.

## Build
```console
unix@weeb:~$ git clone https://github.com/valebes/rsClock
unix@weeb:~$ cd rsClock
unix@weeb:~$ cargo build --release
unix@weeb:~$ ./target/release/rsclock
```
## Usage
```console
usage : rsclock
    -s    Set custom symbol
    -f    Set foreground color [0-255] (Ansi value)
    -b    Set background color [0-255] (Ansi value)
    -d    Debug mode
    -c    Center clock
    -v    Show rsClock version
    -h    Display this message
```
## To-Do
* [ ] Complete code refactory
* [x] Colors support 
* [ ] Alarms
* [ ] Config file
* [ ] Maybe themes support?
## Screenshots

![Screenshot](https://i.imgur.com/EhrFUvk.png)
![Screenshot](https://i.imgur.com/CuirrjG.png)
![Screenshot](https://i.imgur.com/rhaiacW.png)
![Screenshot](https://i.imgur.com/knBYqPb.png)

#### Work in progress
