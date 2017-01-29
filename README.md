# Remote Temperature Measurement with Rust and a Raspberry Pi

## Installation

### Enable the OneWire Linux Subsystem

If you have a DS18B20 temperature sensor, you can connect it, using the schematic
from"https://blog.bandinelli.net/index.php?post/2014/11/23/Temp%C3%A9rature-suivie-avec-un-Raspberry-Pi-B%2C-une-sonde-DS18B20-et-Munin

```sh
sudo raspi-config
```

Go to "9 Advanced Options" then "AA 1-Wire" then "YES" and restart.

### Rust compiler

This projet use the Rust Programming Language on a Raspberry Pi 3.

To install Rust 1.14.0, run the following command:

```sh
curl https://sh.rustup.rs -sSf | sh
rustup default 1.14.0
```

### Source code 

Get the source code somewhere with Git:

```sh
sudo apt install git
git clone git@github.com:BFH-E1D-2015-2016/meteo-backend.git
cd meteo-backend
```

### Compile and run

Using cargo (Rust Package Manager), it's super easy to build and run a project:

```sh
cargo run
```

### Check the website

Goto "http://RASP_IP:8080/static/hello.html" (Replace RASP_IP by the IP addresse of the Raspberry PI).

You can see a chart of all measurement from the sensors

## How it works

Linux know how 1-Wire sensors works, to get a temperature, we just read a file in /sys/bus/w1/devices.

Our propgram use 3 thread:
- One to read the temperature every x secondes and to store it in a SQLite Database 
- One to read the database and create a CSV file with all the database
- One to serve a static website. When someone go to the "hello.html" page, the browser download the CSV
  files and compute a graph using a javascript library



