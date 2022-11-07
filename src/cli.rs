pub fn cli() -> clap::App<'static> {
    clap::app_from_crate!()
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
        clap::App::new("convert")
                .about("convert csv to parquet or vice-versa")
                .arg(
                    clap::arg!(output: -o --output <FORMAT> "select the output file format")
                        .possible_value("parquet")
                )
                .arg(
                    clap::arg!(files: <FILE> ... "path of files to convert")
                )

        )

}