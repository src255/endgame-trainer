use fen::Endgame;
use std::env;
use std::process;

fn main() -> Result<(), &'static str> {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Missing endgame type");
        process::exit(1);
    });

    let endgame = arg.parse::<Endgame>()?;
    println!("{}", endgame.generate_fen());

    Ok(())
}
