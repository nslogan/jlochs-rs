
use std::env;
use std::fs;
use std::io::{self, Write}; // 'Write' is for 'flush()'
// use std::io::prelude::*; // TODO: Why do I need the prelude?
// use std::io::{self, BufRead};

fn main() {
    // Note that std::env::args will panic if any argument contains invalid Unicode
    // TODO(for logan learning rust): What is 'collect' doing?
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() > 2 {
        print_usage();
    } else if args.len() == 2 {
        // TODO: Not sure if I should borrow or move this argument :shrug:
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

// TODO: Add failure argument
fn print_usage() {
    // TODO: Pull program name from args
    println!("Usage: jlochs-rs [FILE]");
}

fn run_file(path: &String) {
    println!("Executing '{}'", path);

    // TODO: Handle this more gracefully than panicking
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    run(&contents);
}

fn run_prompt() {
    let mut input = String::new();

    loop {
        input.clear(); // Clear input buffer
        print!("> ");
        io::stdout().flush().unwrap(); // stdout is frequently line-buffered by default, flush

        match io::stdin().read_line(&mut input) {
            Ok(0) => { // EOF - 'ctrl-d
                println!();
                break;
            }
            Ok(_n) => { // NOTE: Prefix '_' indicates 'n' is unused
                run(&input);
            }
            Err(error) => println!("error: {error}"),
        }        
    }
}

fn run(source: &String) {
    println!("Source: {}", source);

    // Scanner scanner = new Scanner(source);
    // List<Token> tokens = scanner.scanTokens();

    // // For now, just print the tokens.
    // for (Token token : tokens) {
    //     System.out.println(token);
    // }
}
