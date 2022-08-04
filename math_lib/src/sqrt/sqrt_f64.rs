pub(crate) struct Sqrt;

impl Sqrt {
    fn binary_search_sqrt_f64(value: f64, left: f64, right: f64) -> f64 {
        let root = (left + right) / 2.0;
        // println!("root_f: {}, left_f: {}, right_f: {}", root, left, right);
        if root == left || root == right {
            return root as f64;
        }
        if (root * root) as f64 > value {
            Sqrt::binary_search_sqrt_f64(value, left, root)
        } else {
            Sqrt::binary_search_sqrt_f64(value, root, right)
        }
    }

    // pub fn new(n: f64) -> f64 {
    //     Self::custom_sqrt(n)
    // }

    pub(crate) fn custom_sqrt(n: f64) -> f64 {
        Sqrt::binary_search_sqrt_f64(n, 0.0, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_sqrt_works() {
        assert_eq!(Sqrt::custom_sqrt(8.0), 2.82842712474619);
        assert_eq!(Sqrt::custom_sqrt(64.0), 8.0);
    }
}
