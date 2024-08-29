use std::{env, io};
use std::io::{Read, Write};
use std::fs::File;
mod token;
mod tokentype;

struct Lox {
    has_error: bool,
}

fn main() -> io::Result<()> {
    let mut lox = Lox { has_error: false };
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1], &mut lox)?;
    } else {
        run_prompt(&mut lox)?;
    }

    Ok(())
}

fn run_file(path: &String, lox: &mut Lox) -> io::Result<()> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    run(&buffer, lox)?;
    if lox.has_error {
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt(lox: &mut Lox) -> io::Result<()> {
    let mut buffer = String::new();

    loop {
        io::stdout().write(b"> ")?;
        io::stdout().flush()?;

        let num_bytes = io::stdin().read_line(&mut buffer)?;
        if num_bytes == 0 {
            break;
        }
        run(&buffer, lox)?;
        lox.has_error = false;
        buffer.clear();
    }

    Ok(())
}

fn run(source: &String, lox: &mut Lox) -> io::Result<()> {
    let scanner = build_scanner(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}

fn error(line: u32, msg: &String, lox: &mut Lox) {
    report(line, "", msg, lox);
}

fn report(line: u32, whr: &str, msg: &String, lox: &mut Lox) {
    println!("[line {:?}] Error{:?}: {:?}", line, whr, msg);

    lox.has_error = true;
}
