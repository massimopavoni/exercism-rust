use divisors_fixed::Divisors;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    use std::cmp::Ordering::{Equal, Greater, Less};

    Some(match num.cmp(&(num.divisors().iter().sum::<u64>() - num)) {
        Less => Classification::Abundant,
        Equal => Classification::Perfect,
        Greater => Classification::Deficient,
    })
}
