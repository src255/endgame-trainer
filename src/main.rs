use fen::Endgame;
use std::env;

fn main() -> Result<(), &'static str> {
    let arg = env::args().nth(1).ok_or("Missing endgame type")?;
    let endgame = Endgame::try_from(arg)?;

    println!("{}", endgame.generate_fen());

    Ok(())
}
