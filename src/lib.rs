use rand::seq::SliceRandom;
use std::str::FromStr;

pub enum Endgame {
    QueenVsRook,
    RookBishopVsRook,
}

impl Endgame {
    pub fn build(s: &str) -> Result<Self, &'static str> {
        match s.to_ascii_lowercase().as_str() {
            "qr" | "q-r" => Ok(Endgame::QueenVsRook),
            "rbr" | "rb-r" => Ok(Endgame::RookBishopVsRook),
            _ => Err("Unrecognized endgame"),
        }
    }

    pub fn generate_fen(&self) -> String {
        match self {
            Self::QueenVsRook => {
                format!("{} b - -", Q_R.choose(&mut rand::thread_rng()).unwrap())
            }
            Self::RookBishopVsRook => {
                format!("{} w - -", RB_R.choose(&mut rand::thread_rng()).unwrap())
            }
        }
    }
}

impl FromStr for Endgame {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Endgame::build(s)
    }
}

const Q_R: [&str; 8] = [
    "1k6/1r6/2K5/Q7/8/8/8/8",
    "6k1/6r1/5K2/7Q/8/8/8/8",
    "3Q4/kr6/2K5/8/8/8/8/8",
    "4Q3/6rk/5K2/8/8/8/8/8",
    "8/8/8/8/8/2K5/kr6/3Q4",
    "8/8/8/8/8/5K2/6rk/4Q3",
    "8/8/8/8/Q7/2K5/1r6/1k6",
    "8/8/8/8/7Q/5K2/6r1/6k1",
];

const RB_R: [&str; 1] = ["3k4/4r3/3K4/3B4/8/8/8/5R2"];
