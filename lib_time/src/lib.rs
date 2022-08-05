pub mod now;
use std::time::SystemTime;

use now::get_time::*;

pub fn bench_lib_now() -> f64 {
    let second: f64 = get_time_now();
    second as f64
}

pub fn bench_lib_now_inline() -> f64 {
    let start_time_duration = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| panic!("{}", e));
    let now_duration = start_time_duration;
    now_duration.as_secs_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_benches() {
        assert_eq!(bench_lib_now().round(), bench_lib_now_inline().round());
    }
}
