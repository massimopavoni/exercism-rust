use itertools::Itertools;

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> Self {
        Self {
            rails: rails as usize,
        }
    }

    fn zigzag_iter(&self) -> impl Iterator<Item = usize> {
        (0..self.rails - 1).chain((1..self.rails).rev()).cycle()
    }

    pub fn encode(&self, text: &str) -> String {
        let mut cipher = vec![String::new(); self.rails];

        for (text_c, rail_i) in text.chars().zip(self.zigzag_iter()) {
            cipher[rail_i].push(text_c);
        }

        cipher.concat()
    }

    pub fn decode(&self, cipher: &str) -> String {
        self.zigzag_iter()
            .zip(0..)
            .take(cipher.len())
            .sorted_unstable()
            .zip(cipher.chars())
            .map(|((_, i), c)| (i, c))
            .sorted_unstable()
            .map(|(_, c)| c)
            .collect()
    }
}
