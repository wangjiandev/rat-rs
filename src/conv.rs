use crate::frac::Fraction;

impl From<u32> for Fraction {
    fn from(value: u32) -> Self {
        Self::with_positive(value, 1).unwrap()
    }
}

impl From<i32> for Fraction {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self::with_negative(value.unsigned_abs(), 1).unwrap()
        } else {
            Self::with_positive(value as u32, 1).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::frac::Fraction;

    #[test]
    fn test_from_u32() {
        assert_eq!(
            Fraction::from(2_u32),
            Fraction::with_positive(2, 1).unwrap()
        );
        assert_eq!(Fraction::from(5), Fraction::with_positive(5, 1).unwrap());
        assert_eq!(Fraction::from(-7), Fraction::with_negative(7, 1).unwrap());
        assert_eq!(Fraction::from(0), Fraction::with_positive(0, 1).unwrap());
    }
}
