mod cli;
mod command;

fn main() {
    let cli = cli::cli().get_matches();

    match &cli.subcommand() {
        Some(("convert", args)) => {command::convert(args)}
        _ => {println!("check help")}
    }


}
