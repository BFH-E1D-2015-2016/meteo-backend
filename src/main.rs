extern crate glob;
extern crate regex;

use std::str::FromStr;
use std::path::PathBuf;
use regex::Regex;
use std::fs::File;
use std::io::Read;

fn read_sensor() -> Result<f32, ()> {

    let data = search_sensors();
    for path in data {
        let mut file = File::open(path).unwrap();
        let mut text: Vec<u8> = Vec::with_capacity(75);

        let re_crc = Regex::new(r"(YES)").unwrap();

        let _ = file.read_to_end(&mut text);

        if re_crc.is_match(std::str::from_utf8(&text).unwrap()) {
            let re_temp = Regex::new(r"t=(-?\d+)").unwrap();

            match re_temp.captures(std::str::from_utf8(&text).unwrap()).unwrap().at(1) {
                Some(tmp) => {
                    let temperature = f32::from_str(tmp).unwrap();
                    return Ok(temperature / 1000.0);
                }
                None => {}
            };
        }
    }

    Err(())

}
fn search_sensors() -> Vec<PathBuf> {

    glob::glob("/sys/bus/w1/devices/28-*/w1_slave").unwrap().filter_map(Result::ok).collect()
}


fn main() {
    println!("{:?}", read_sensor());
}
