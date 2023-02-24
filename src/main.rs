use fen::{Endgame, FenError};
use std::env;

fn main() -> Result<(), FenError> {
    let arg = env::args().nth(1).ok_or("Missing endgame type")?;
    let endgame = Endgame::try_from(arg)?;

    println!("{}", endgame.generate_fen());

    Ok(())
}
