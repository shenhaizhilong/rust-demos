use std::thread;
use std::time::{Duration, Instant};
use chrono::{DateTime, Local, TimeZone, Utc};

fn main() {}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::{Duration, Instant};
    use chrono::{DateTime, Local, TimeZone, Utc};

    #[test]
    fn test1() {
        let start = Instant::now();
        thread::sleep(Duration::from_secs(3));
        let duration = start.elapsed();
        println!("elapsed: {:?}s", duration.as_secs());
    }


    #[test]
    fn test2() {
        let local_time = Local::now();
        let utc = Utc::now();
        println!("{:?}", local_time);
        println!("{:?}", utc);
        println!("{:?}", utc.timestamp_millis());


        let dt = Utc.with_ymd_and_hms(2024, 06, 8, 10, 12, 0).unwrap();
        println!("{:?}", dt);
        println!("{:?}", dt.timestamp_millis());

        let ts = DateTime::from_timestamp_millis(947638923004).expect("invalid timestamp");
        assert_eq!(ts.to_string(), "2000-01-12 01:02:03.004 UTC");
        assert_eq!(DateTime::from_timestamp_millis(ts.timestamp_millis()).unwrap(), ts);
    }
}
