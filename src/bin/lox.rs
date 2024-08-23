use std::{env, io};
fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        runFile(&args[1]);
    } else {
        runPrompt();
    }

    Ok(())
}
