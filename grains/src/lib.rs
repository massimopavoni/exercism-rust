pub fn square(s: u32) -> u64 {
    if !(1..65).contains(&s) {
        panic!("Square must be between 1 and 64");
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // Possible variable solution
    // (1..65).map(square).sum()

    // Hilarious solution
    u64::MAX
}
