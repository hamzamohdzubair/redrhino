mod convert;

pub fn convert(args: &clap::ArgMatches) {
    match args.value_of("output").unwrap() {
        "parquet" => {convert::create_parquet(&args)}
        _ => {println!("Not a valid input")}
    }
}