use std::sync::{
    atomic::{AtomicUsize, Ordering},
    LazyLock,
};

use itertools::Itertools;
use rand::seq::SliceRandom;

pub struct Robot<'a> {
    name: &'a str,
}

static ROBOT_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut rng = rand::rngs::ThreadRng::default();

    let mut numbers = (0..=999).map(|n| format!("{:03}", n)).collect_vec();
    numbers.shuffle(&mut rng);

    let mut alphas: Vec<String> = (b'A'..=b'Z')
        .combinations_with_replacement(2)
        .map(|bs| String::from_utf8(bs).expect("Expected valid utf8"))
        .collect();
    alphas.shuffle(&mut rng);

    alphas
        .iter()
        .flat_map(|a| numbers.iter().map(|n| (a.clone() + n)).collect_vec())
        .collect_vec()
});

static CURRENT_NAME: AtomicUsize = AtomicUsize::new(0);

impl<'a> Robot<'a> {
    pub fn new() -> Self {
        Self {
            name: &ROBOT_NAMES[CURRENT_NAME.fetch_add(1, Ordering::SeqCst)],
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn reset_name(&mut self) {
        self.name = &ROBOT_NAMES[CURRENT_NAME.fetch_add(1, Ordering::SeqCst)];
    }
}
