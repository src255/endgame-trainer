use fen::Endgame;
use std::env;

fn main() -> Result<(), &'static str> {
    let mut args = env::args();
    args.next();

    let arg = args.next().expect("Should receive endgame type");

    let endgame = Endgame::build(&arg)?;
    println!("{}", endgame.generate_fen());

    Ok(())
}
