pub mod now;
use now::get_time;

pub fn bench_lib_now() {
    get_time::get_time_now();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
