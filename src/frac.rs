/// Numerator / Denominator
#[derive(Debug, PartialEq, Eq)]
pub struct Fraction {
    numer: u32,
    denom: u32,
    sign: FractionSign,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FractionSign {
    Positive,
    Negative,
}

impl Fraction {
    pub fn new(numer: u32, denom: u32, sign: FractionSign) -> Option<Self> {
        if denom == 0 {
            return None;
        }
        let gcd = gcd(numer, denom);
        Some(Fraction {
            numer: numer / gcd,
            denom: denom / gcd,
            sign,
        })
    }

    pub fn with_positive(numer: u32, denom: u32) -> Option<Self> {
        Some(Fraction {
            numer,
            denom,
            sign: FractionSign::Positive,
        })
    }

    pub fn with_negative(numer: u32, denom: u32) -> Option<Self> {
        Some(Fraction {
            numer,
            denom,
            sign: FractionSign::Negative,
        })
    }
}

fn gcd(mut m: u32, mut n: u32) -> u32 {
    while n != 0 {
        let remainder = m % n;
        m = core::mem::replace(&mut n, remainder);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(21, 14), 7);
        assert_eq!(gcd(105, 91), 7);
        assert_eq!(gcd(100, 0), 100);
        assert_eq!(gcd(0, 100), 100);
        assert_eq!(gcd(21, 1), 1);
        assert_eq!(gcd(1, 21), 1);
    }

    #[test]
    fn test_new() {
        assert_eq!(
            Fraction::new(1, 2, FractionSign::Negative),
            Some(Fraction {
                numer: 1,
                denom: 2,
                sign: FractionSign::Negative
            })
        );
        assert_eq!(
            Fraction::new(1, 2, FractionSign::Positive),
            Some(Fraction {
                numer: 1,
                denom: 2,
                sign: FractionSign::Positive
            })
        );
        assert_eq!(
            Fraction::new(2, 4, FractionSign::Negative),
            Some(Fraction {
                numer: 1,
                denom: 2,
                sign: FractionSign::Negative
            })
        );
        assert_eq!(
            Fraction::new(2, 4, FractionSign::Positive),
            Some(Fraction {
                numer: 1,
                denom: 2,
                sign: FractionSign::Positive
            })
        );
        assert_eq!(Fraction::new(3, 0, FractionSign::Positive), None);
        assert_eq!(Fraction::new(3, 0, FractionSign::Negative), None);
        assert_eq!(
            Fraction::new(0, 3, FractionSign::Positive),
            Some(Fraction {
                numer: 0,
                denom: 1,
                sign: FractionSign::Positive
            })
        );
        assert_eq!(
            Fraction::new(0, 3, FractionSign::Negative),
            Some(Fraction {
                numer: 0,
                denom: 1,
                sign: FractionSign::Negative
            })
        );
        assert_eq!(
            Fraction::new(21, 14, FractionSign::Negative),
            Some(Fraction {
                numer: 3,
                denom: 2,
                sign: FractionSign::Negative
            })
        );
        assert_eq!(
            Fraction::new(21, 14, FractionSign::Positive),
            Some(Fraction {
                numer: 3,
                denom: 2,
                sign: FractionSign::Positive
            })
        );
        assert_eq!(
            Fraction::new(21, 1, FractionSign::Positive),
            Some(Fraction {
                numer: 21,
                denom: 1,
                sign: FractionSign::Positive
            })
        );
        assert_eq!(
            Fraction::new(21, 1, FractionSign::Negative),
            Some(Fraction {
                numer: 21,
                denom: 1,
                sign: FractionSign::Negative
            })
        );
    }
}
