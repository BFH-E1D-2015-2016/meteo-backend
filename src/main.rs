extern crate glob;

use std::path::PathBuf;

fn search_sensors() -> Vec<PathBuf> {
    glob::glob("/sys/bus/w1/devices/28-*/w1_slave").unwrap().filter_map(Result::ok).collect()
}

fn main() {



    println!("{:?}", search_sensors());
}
