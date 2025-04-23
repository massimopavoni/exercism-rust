const START: usize = 2;

pub fn primes_up_to(n: u64) -> Vec<u64> {
    let upper_bound = n as usize;

    if upper_bound < START {
        return Vec::new();
    }

    {
        let mut is_prime = vec![true; upper_bound + 1 - START];
        let limit = (upper_bound as f64).sqrt().ceil() as usize;

        for i in START..limit {
            let mut it = is_prime[i - START..].iter_mut().step_by(i);

            if let Some(true) = it.next() {
                it.for_each(|x| *x = false);
            }
        }
        
        is_prime
    }
    .into_iter()
    .enumerate()
    .filter_map(|(e, b)| if b { Some((e + START) as u64) } else { None })
    .collect()
}
