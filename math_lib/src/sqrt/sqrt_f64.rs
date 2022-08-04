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
    use anyhow::Result;
    use roots::find_roots_linear;
    use roots::Roots;

    #[test]
    fn test_it_works() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_sqrt_works() {
        assert_eq!(Sqrt::custom_sqrt(8.0), 2.82842712474619);
        assert_eq!(Sqrt::custom_sqrt(64.0), 8.0);
    }

    #[test]
    fn test_sqrt_custom_perfect_sqrt() {
        let number = 100.0;
        let sqrt = Sqrt::custom_sqrt(number);
        // Solves a linear equation a1*x + a0 = 0.
        // Returns Roots::Two([0f64]) as '1*x + 0 = 0' has the root 0
        let root_linear = find_roots_linear(sqrt, -number);
        assert_eq!(sqrt, 10.0);
        assert_eq!(root_linear, Roots::One([sqrt]));
    }

    #[test]
    fn test_sqrt_custom_std_sqrt() {
        let number = 100.0;
        let custom_sqrt = Sqrt::custom_sqrt(number);
        let std_method_sqrt: f64 = number.sqrt();
        assert_eq!(custom_sqrt, std_method_sqrt);
    }

    #[test]
    fn test_sqrt_custom_std_iter_sqrt() {

        let malloc: f64 = 100_000.0;
        let usize: usize = malloc as usize;
        let mut v: Vec<f64> = Vec::with_capacity(usize);

        for i in 0..malloc as i32 {
            let j: f64 = Result::unwrap(TryInto::try_into(i));
            v.push(j);
        }

        let w: Vec<f64> = v.to_owned();
        for x in IntoIterator::into_iter(w) {
            let number: f64 = x;
            let custom_sqrt: f64 = Sqrt::custom_sqrt(number);
            let std_method_sqrt: f64 = number.sqrt();
            assert_eq!(custom_sqrt.round(), std_method_sqrt.round());
            assert_eq!(next_char(custom_sqrt), next_char(std_method_sqrt));
            assert_eq!(next_char(custom_sqrt), next_char(std_method_sqrt));
        }

        fn next_char(float: f64) -> char {
            float.to_string().chars().next().unwrap()
        }
    }  
}
