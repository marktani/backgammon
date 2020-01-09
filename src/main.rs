use clap::{App, Arg, SubCommand};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("Backgammon")
        .version("1.0")
        .author("Nilan Marktanner <nilan.marktanner@gmail.com>")
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("new")
                .about("Starts a new game")
                .version("1.0"),
        )
        .subcommand(
            SubCommand::with_name("move")
                .about("Move token on specified point")
                .version("1.0")
                .arg(Arg::with_name("point").short("p").help("Point to move")),
        )
        .subcommand(
            SubCommand::with_name("print")
                .about("Print the current status")
                .version("1.0"),
        )
        .get_matches();

    match matches.subcommand {
        Some(ref subcommand) if subcommand.name == "new" => {
            println!("Do you really want to start a new game? (y/N)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            println!("{}", input.len());
            match input.to_lowercase().trim_end().as_ref() {
                "yes" | "y" => {
                    // restart game
                    println!("Starting a new game...")
                }
                _ => {
                    // don't restart game
                    println!("Keep on playing!")
                }
            }
        }
        Some(ref subcommand) if subcommand.name == "move" => {}
        Some(ref subcommand) if subcommand.name == "print" => {}
        Some(_) => {}
        None => println!("none"),
    };
    println!("Done");

    Ok(())
}
