use std::time::SystemTime;

pub(crate) fn get_time_now() -> f64 {
    let start_time_duration = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| panic!("{}", e));
    let now_duration = start_time_duration;
    now_duration.as_secs_f64()
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1f64, 1f64);
    }

    #[test]
    fn it_gets_time_now() {
        let present = get_time_now();
        assert_eq!(present.round(), get_time_now().round());
    }

    #[test]
    fn test_now_if_sleep_ms() {
        let now_1: f64 = get_time_now();
        sleep(Duration::from_secs_f64(2f64));
        let now_2: f64 = get_time_now();
        let time_diff: f64 = now_2 - now_1;

        assert_eq!(time_diff.round(), 2f64);
    }
    #[test]
    fn test_now_chrono() {
        let present = get_time_now();
        assert_eq!(present.round(), get_time_now().round());
    }
}

// pub(crate) fn it_gets_frame_time() {
// miniquad::date::now() - now
