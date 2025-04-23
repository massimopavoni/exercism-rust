use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::{Display, Formatter},
};

struct Team {
    name: String,
    matches: u8,
    wins: u8,
    draws: u8,
    losses: u8,
    points: u8,
}

impl Team {
    fn new(name: &str) -> Self {
        Team {
            name: name.to_string(),
            matches: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }

    fn win(&mut self) {
        self.matches += 1;
        self.wins += 1;
        self.points += 3;
    }

    fn loss(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }

    fn draw(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name, self.matches, self.wins, self.draws, self.losses, self.points
        ))
    }
}

pub fn tally(match_results: &str) -> String {
    let tally = String::from("Team                           | MP |  W |  D |  L |  P");

    if match_results.is_empty() {
        return tally;
    }

    let mut results = HashMap::new();

    for match_result in match_results.lines() {
        let [team1, team2, result] = match_result.split(';').collect::<Vec<_>>()[..] else {
            panic!("Invalid match result");
        };

        match result {
            "win" => {
                results
                    .entry(team1)
                    .or_insert_with(|| Team::new(team1))
                    .win();
                results
                    .entry(team2)
                    .or_insert_with(|| Team::new(team2))
                    .loss();
            }
            "loss" => {
                results
                    .entry(team1)
                    .or_insert_with(|| Team::new(team1))
                    .loss();
                results
                    .entry(team2)
                    .or_insert_with(|| Team::new(team2))
                    .win();
            }
            "draw" => {
                results
                    .entry(team1)
                    .or_insert_with(|| Team::new(team1))
                    .draw();
                results
                    .entry(team2)
                    .or_insert_with(|| Team::new(team2))
                    .draw();
            }
            _ => panic!("Invalid result"),
        }
    }

    let mut results = results.values().collect::<Vec<_>>();
    results.sort_by_key(|t| (Reverse(t.points), &t.name));

    tally
        + "\n"
        + &results
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n")
}
