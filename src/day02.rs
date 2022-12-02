#[derive(Eq, PartialEq, Clone, Debug)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn fight(&self, other: &Self) -> u32 {
        match (self, other) {
            (x, y) if x == y => self.points() + 3,
            (RPS::Rock, RPS::Paper) => self.points(),
            (RPS::Rock, RPS::Scissors) => self.points() + 6,
            (RPS::Paper, RPS::Rock) => self.points() + 6,
            (RPS::Paper, RPS::Scissors) => self.points(),
            (RPS::Scissors, RPS::Rock) => self.points(),
            (RPS::Scissors, RPS::Paper) => self.points() + 6,
            _ => 1337,
        }
    }

    pub fn fight2(&self, other: &Self) -> u32 {
        match (self, other) {
            (x, RPS::Rock) => x.get_loss().points(),
            (x, RPS::Paper) => x.points() + 3,
            (x, RPS::Scissors) => x.get_win().points() + 6,
        }
    }
    pub fn points(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    pub fn get_win(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
    pub fn get_loss(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

pub fn parse(input: &str) -> Vec<(RPS, RPS)> {
    input
        .lines()
        .map(|e| {
            let mut f = e.trim().split(' ');
            let x = match f.next() {
                Some("A") => RPS::Rock,
                Some("B") => RPS::Paper,
                Some("C") => RPS::Scissors,
                _ => RPS::Rock,
            };
            let y = match f.next() {
                Some("X") => RPS::Rock,
                Some("Y") => RPS::Paper,
                Some("Z") => RPS::Scissors,
                _ => RPS::Rock,
            };
            (x, y)
        })
        .collect::<Vec<(RPS, RPS)>>()
}

pub fn part1(input: &mut [(RPS, RPS)]) -> u32 {
    input.iter().map(|(o, s)| s.fight(o)).sum::<u32>()
}

pub fn part2(input: &mut [(RPS, RPS)]) -> u32 {
    input.iter().map(|(o, s)| o.fight2(s)).sum::<u32>()
}
