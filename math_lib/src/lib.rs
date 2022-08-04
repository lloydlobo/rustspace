pub(crate) mod sqrt;
pub(crate) use sqrt::sqrt_f64::Sqrt;

pub fn math_lib_sqrt(number: f64) {
    Sqrt::custom_sqrt(number);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
