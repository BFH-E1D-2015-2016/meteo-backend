use std::fs::OpenOptions;

use rusqlite::Connection;

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
            let data: (i64, i64) =
        (
            row.get(0),
            row.get(1)
        );
        data
    }).unwrap();

    for tmp in tmp_iter {
        println!("Found person {:?}", tmp.unwrap());
    }



        //for line in lines {
        //    write!(&mut file, "{}, {}", date, value);
        //}


        ::std::thread::sleep(::std::time::Duration::from_secs(7));

    }

}
