use std::error::Error;
use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};

fn main() -> Result<(), Box<dyn Error>> {
    let dt: DateTime<Utc> = DateTime::parse_from_rfc3339("2026-02-17T21:09:00.000000000Z")?.with_timezone(&Utc);


    println!("time: {}", dt.timestamp());
    println!("exam: 1688671200");

    Ok(())
}
