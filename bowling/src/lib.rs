#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    throws: Vec<u16>,
    second_throw: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second_throw && pins + self.throws.last().unwrap() > 10) {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.score().is_some() {
            return Err(Error::GameComplete);
        }

        self.throws.push(pins);

        self.second_throw = !(self.second_throw || pins == 10);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut frame = 0;
        let mut total = 0;
        let mut score;

        for _ in 1..=10 {
            score = self.throws.get(frame)? + self.throws.get(frame + 1)?;
            total += score;

            if score >= 10 {
                total += self.throws.get(frame + 2)?;
            }

            frame += if self.throws[frame] == 10 { 1 } else { 2 };
        }

        Some(total)
    }
}
