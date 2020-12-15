use chrono::{NaiveDate, Datelike, Weekday};

fn main() {
    println!("Hello, world!");

    let mut year = 1901;
    let mut month = 1;
    let mut count = 0;

    while year < 2001 {

    	month = 1;
    	while month <= 12 {
    		let d = NaiveDate::from_ymd(year, month, 1);

    		if d.weekday() == Weekday::Sun {
    			count += 1;
    		}
    		month += 1;
    	}

    	year += 1;
    }

    println!("{:?}", count);
}
