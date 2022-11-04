use std::{cmp::Ordering, collections::HashMap};

#[derive(Default, Debug)]
struct Score {
    wins: u32,
    draws: u32,
    losses: u32,
}

impl Score {
    fn new(wins: u32, draws: u32, losses: u32) -> Self {
        Self {
            wins,
            draws,
            losses,
        }
    }

    fn matches_played(&self) -> u32 {
        self.wins + self.draws + self.losses
    }

    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }
}

fn compute_result(teams: &mut HashMap<String, Score>, result: &str) {
    let mut parts = result.split(";");
    let home_team_name = parts.next().unwrap().to_string();
    let visitor_team_name = parts.next().unwrap().to_string();
    let result = parts.next().unwrap();

    match result {
        "win" => {
            teams
                .entry(home_team_name)
                .and_modify(|e| e.wins += 1)
                .or_insert(Score::new(1, 0, 0));
            teams
                .entry(visitor_team_name)
                .and_modify(|e| e.losses += 1)
                .or_insert(Score::new(0, 0, 1));
        }
        "loss" => {
            teams
                .entry(home_team_name)
                .and_modify(|e| e.losses += 1)
                .or_insert(Score::new(0, 0, 1));
            teams
                .entry(visitor_team_name)
                .and_modify(|e| e.wins += 1)
                .or_insert(Score::new(1, 0, 0));
        }
        _ => {
            teams
                .entry(home_team_name)
                .and_modify(|e| e.draws += 1)
                .or_insert(Score::new(0, 1, 0));
            teams
                .entry(visitor_team_name)
                .and_modify(|e| e.draws += 1)
                .or_insert(Score::new(0, 1, 0));
        }
    };

    dbg!(teams);
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Score> = HashMap::new();
    for result in match_results.lines() {
        compute_result(&mut teams, result)
    }
    let mut teams_vec: Vec<(String, Score)> = teams.into_iter().collect();
    teams_vec.sort_by(|a, b| {
        let mut cmp = b.1.points().cmp(&a.1.points());
        if cmp == Ordering::Equal {
            cmp = a.0.cmp(&b.0);
        }

        cmp
    });

    let mut table = vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    for (name, score) in teams_vec {
        table.push(format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            name,
            score.matches_played(),
            score.wins,
            score.draws,
            score.losses,
            score.points()
        ))
    }

    table.join("\n")
}
