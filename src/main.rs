extern crate glob;
extern crate regex;

use std::path::PathBuf;
use regex::Regex;
use std::fs::File;
use std::io::Read;

fn search_sensors() -> Vec<PathBuf> {

    glob::glob("/sys/bus/w1/devices/28-*/w1_slave").unwrap().filter_map(Result::ok).collect()
}

fn main() {
    let data = search_sensors();
    for path in data {
        let mut file = File::open(path).unwrap();
        let mut text: Vec<u8> = Vec::with_capacity(75);

        let re_crc = Regex::new(r"(YES)").unwrap();

        file.read_to_end(&mut text);

        if re_crc.is_match(std::str::from_utf8(&text).unwrap()) {
            let re_temp = Regex::new(r"t=(-?\d+)").unwrap();

            match re_temp.captures(std::str::from_utf8(&text).unwrap()).unwrap().at(1) {
                Some(tmp) => println!("{}", tmp),
                None => {}
            };
        }


    }
    println!("{:?}", search_sensors());
}
