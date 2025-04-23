pub fn nth(n: usize) -> u32 {
    let mut primes = vec![2];
    let mut i = 3;

    while primes.len() < n + 1 {
        let i_sqrt = (i as f64).sqrt().ceil() as u32;

        if primes
            .iter()
            .take_while(|&&p| p <= i_sqrt)
            .all(|&p| i % p != 0)
        {
            primes.push(i);
        }

        i += 2;
    }

    primes[n]
}
