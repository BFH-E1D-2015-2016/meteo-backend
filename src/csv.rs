use std::fs::OpenOptions;

use rusqlite::Connection;
use std::io::Write;

use chrono::{NaiveDateTime, NaiveDate};
use chrono::Datelike;
use chrono::Timelike;
pub fn run() {

    loop {
        let conn = Connection::open("db.sqlite3").unwrap();

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("static/data.csv")
            .unwrap();

        let mut stmt = conn.prepare("SELECT timestmp, value FROM tmp").unwrap();
        let mut tmp_iter = stmt.query_map(&[], |row| {
                let data: (i64, i64) = (row.get(0), row.get(1));
                data
            })
            .unwrap();

        for tmp in tmp_iter {
            let (timestmp, value) = tmp.unwrap();
            let time = NaiveDateTime::from_timestamp(timestmp, 0);
            write!(&file,
                   "{:02}/{:02}/{:02} {:02}:{:02}:{}, {}\n",
                   time.year(),
                   time.month(),
                   time.day(),
                   time.hour(),
                   time.minute(),
                   time.second(),
                   (value as f64) / 1000.0);
        }





        ::std::thread::sleep(::std::time::Duration::from_secs(7));

    }

}
