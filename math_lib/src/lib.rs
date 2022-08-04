pub(crate) mod sqrt;
pub(crate) use sqrt::sqrt_f64::Sqrt;

pub fn math_lib_sqrt(number: f64) {
    Sqrt::custom_sqrt(number);
}

#[cfg(test)]
mod tests {

    use std::{
        thread::{self, sleep},
        time::{self, Duration},
    };

    use criterion::black_box;
    use log::trace;

    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn bench_math_lib_sqrt() {
        let time_ms: u64 = 10;
        let ten_millis: Duration = time::Duration::from_millis(time_ms);

        let time_start: time::Instant = time::Instant::now();
        thread::sleep(ten_millis);
        trace!("now({}): {:?}", time_ms, time_start);

        math_lib_sqrt(black_box(100.0));

        let time_end: time::Instant = time::Instant::now();
        sleep(Duration::from_millis(time_ms));
        trace!("now({}): {:?}", time_ms, time_end);

        let time_elapsed: Duration = time_end - time_start;
        let time_elapsed_ms: u128 = time_elapsed.as_millis();
        println!("{}", time_elapsed_ms);
        assert_ne!(time_elapsed_ms, 100);
        assert_eq!(time_elapsed_ms, 10);
    }
}
