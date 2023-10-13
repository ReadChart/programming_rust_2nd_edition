use num::Complex;

pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match super::parse_pair_util::parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[cfg(test)]
mod parse_complex_tests {
    use num::Complex;

    #[test]
    fn parse_complex_tests() {
        assert_eq!(crate::parse_complex_util::parse_complex("1.5,-0.21"), Some(Complex { re: 1.5, im: -0.21 }));
        assert_eq!(crate::parse_complex_util::parse_complex(",-0.21"), None);
    }
}