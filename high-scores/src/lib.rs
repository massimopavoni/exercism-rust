#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top = [0, 0, 0];

        for &score in self.scores {
            if score > top[0] {
                top[2] = top[1];
                top[1] = top[0];
                top[0] = score;
            } else if score > top[1] {
                top[2] = top[1];
                top[1] = score;
            } else if score > top[2] {
                top[2] = score;
            }
        }

        top.iter().filter(|&x| *x > 0).copied().collect()
    }
}
