use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies {
    score: u32,
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score,
            allergies: Allergen::iter()
                .filter(|a| Self::score_and_allergen(score, a))
                .collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        Self::score_and_allergen(self.score, allergen)
    }

    fn score_and_allergen(score: u32, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        score & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
