use std::env;
use minigrep::run;

fn main() {
    let args_os = env::args().collect::<Vec<_>>();

    let conf = minigrep::Config::new(&args_os).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("{conf}");

    run(conf).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    })
}

