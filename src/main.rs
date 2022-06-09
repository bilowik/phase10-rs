use cli_table::{format, print_stdout, Cell, Style, Table};
use std::io::{self, Write};
use std::iter::Extend;
use std::str::FromStr;
use structopt::StructOpt;

fn get_in<T, F: Fn(&str) -> Result<T, String>>(prompt: &str, f: F) -> T {
    loop {
        print!("{}: ", prompt);
        std::io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match f(&s[0..s.len() - 1]) {
            Ok(v) => {
                return v;
            }
            Err(s) => println!("{}. Please try again", &s),
        }
    }
}

fn print_scores(players: &Vec<Player>) {
    let mut table = vec![];
    let title_row = std::iter::once("Round".cell().bold(true))
        .chain(
            players
                .iter()
                .map(|p| format!("{}: {}", p.name.clone(), p.phase()).cell().bold(true)),
        )
        .collect::<Vec<_>>();

    let num_rounds = players[0].get_rounds().len();
    for i in 0..num_rounds {
        table.push(
            std::iter::once(format!("Round {}", i + 1).cell().bold(true))
                .chain(players.iter().map(|p| {
                    let round = p.get_round(i);
                    let mut ret = round.score.to_string();
                    if round.phased_up {
                        ret.push('+');
                    }
                    ret.cell()
                }))
                .collect(),
        )
    }
    table.push(Vec::new());
    table.push(
        std::iter::once("".cell()).chain(
        players
            .iter()
            .map(|p| format!("Total: {}", p.total_score()).cell().bold(true)))
            .collect(),
    );
    print_stdout(table.table().title(title_row));
}

fn main() {
    let args = Args::from_args();
    let mut players = args
        .players
        .into_iter()
        .map(|x| Player::new(&x))
        .collect::<Vec<Player>>();

    loop {
        print_scores(&players);
        if let Some(player) = players
            .iter()
            .filter(|p| p.phase() == 11)
            .min_by(|p1, p2| p1.total_score().cmp(&p2.total_score()))
        {
            println!("{} wins!", &player.name);
            break;
        }

        get_in("Press enter when the round has finished", |_| Ok(()));

        for player in players.iter_mut() {
            let phased_up = get_in(&format!("Did {} phase up?", &player.name), |s| {
                if s == "y" {
                    Ok(true)
                } else if s == "n" {
                    Ok(false)
                } else {
                    Err(String::from("Only accepts 'y' or 'n'"))
                }
            });

            let score = get_in(&format!("Enter score for {}", &player.name), |s| {
                usize::from_str(&s).map_err(|_| format!("'{}' is not a number", &s))
            });

            player.add_round(score, phased_up);
        }
    }
}

struct Round {
    pub score: usize,
    pub phased_up: bool,
}

impl Round {
    pub fn new(score: usize, phased_up: bool) -> Self {
        Self { score, phased_up }
    }

    pub fn won(&self) -> bool {
        self.score == 0
    }
}

struct Player {
    name: String,
    rounds: Vec<Round>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            rounds: Vec::new(),
        }
    }

    pub fn phase(&self) -> u8 {
        self.rounds.iter().filter(|round| round.phased_up).count() as u8 + 1
    }

    pub fn total_score(&self) -> usize {
        self.rounds
            .iter()
            .map(|round| round.score)
            .fold(0, |total, score| total + score)
    }

    pub fn add_round(&mut self, score: usize, phased_up: bool) {
        self.rounds.push(Round::new(score, phased_up));
    }

    pub fn get_round(&self, i: usize) -> &Round {
        &self.rounds[i]
    }

    pub fn get_rounds(&self) -> &Vec<Round> {
        &self.rounds
    }
}

#[derive(StructOpt, Debug)]
struct Args {
    players: Vec<String>,
}
