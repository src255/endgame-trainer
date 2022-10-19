use rand::Rng;

pub enum Endgame {
    QueenVsRook,
    RookBishopVsRook,
}

impl Endgame {
    pub fn build(name: &str) -> Result<Endgame, &'static str> {
        match name {
            "q-r" => Ok(Endgame::QueenVsRook),
            "rb-r" => Ok(Endgame::RookBishopVsRook),
            _ => Err("Unrecognized endgame type"),
        }
    }

    pub fn generate_fen(&self) -> String {
        let mut rng = rand::thread_rng();
        match self {
            Self::QueenVsRook => {
                let i = rng.gen_range(0..Q_R.len());
                format!("{} b - -", Q_R[i])
            }
            Self::RookBishopVsRook => {
                let i = rng.gen_range(0..RB_R.len());
                format!("{} w - -", RB_R[i])
            }
        }
    }
}

const Q_R: [&str; 8] = [
    "1k6/1r6/2K5/Q7/8/8/8/8",
    "6k1/6r1/5k2/7Q/8/8/8/8",
    "3Q4/kr6/2K5/8/8/8/8/8",
    "4Q3/6rk/5K2/8/8/8/8/8",
    "8/8/8/8/8/2K5/kr6/3Q4",
    "8/8/8/8/8/5K2/6rk/4Q3",
    "8/8/8/8/Q7/2K5/1r6/1k6",
    "8/8/8/8/7Q/5K2/6r1/6k1",
];

const RB_R: [&str; 1] = ["3k4/4r3/3K4/3B4/8/8/8/5R2"];
