use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(mut dice: Dice, category: Category) -> u8 {
    let dice_counts = dice.iter().fold(HashMap::new(), |mut map, &d| {
        *map.entry(d).or_insert(0_u8) += 1;
        map
    });

    match category {
        Category::FullHouse => {
            if dice_counts.len() == 2 && dice_counts.values().any(|&v| v == 2) {
                dice.iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => dice_counts
            .iter()
            .find(|&(_, &v)| v >= 4)
            .map_or(0, |(&k, _)| k * 4),
        Category::LittleStraight => {
            dice.sort_unstable();

            if dice_counts.len() == 5 && dice[0] == 1 && dice[4] == 5 {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            dice.sort_unstable();

            if dice_counts.len() == 5 && dice[0] == 2 && dice[4] == 6 {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if dice_counts.len() == 1 {
                50
            } else {
                0
            }
        }
        other => {
            let value = match other {
                Category::Ones => 1,
                Category::Twos => 2,
                Category::Threes => 3,
                Category::Fours => 4,
                Category::Fives => 5,
                Category::Sixes => 6,
                _ => unreachable!(),
            };
            *dice_counts.get(&value).unwrap_or(&0) * value
        }
    }
}
