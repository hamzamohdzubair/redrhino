use polars::prelude::*;
use colored::*;

pub fn create_parquet(args: &clap::ArgMatches) {
    for file in args.values_of("files").unwrap() {

        let bar = indicatif::ProgressBar::new(10)
            .with_message("Converting")
            .with_style(indicatif::ProgressStyle::default_bar()
                .template("{msg:>15.cyan} {bar:40.cyan} {prefix} {percent:>3}%  [{elapsed}]")
                )
                ;
            
        bar.set_prefix(format!("{}", file));
        bar.inc(1);
        let df = CsvReader::from_path(file)
            .unwrap()
            .infer_schema(None)
            .has_header(true)
            .finish()
            .unwrap()
            ;

        bar.inc(6);

        let filename_parquet = std::path::Path::new(file).with_extension("parquet");
        
        let file_parquet = std::fs::File::create(&filename_parquet)
            .expect("Could not create file");
        
        ParquetWriter::new(file_parquet)
            .finish(&df)
            .unwrap()
            ;

        bar.inc(3);
        bar.finish_with_message("Converted");
        println!("{:>15} {}", "Created".green().bold(), filename_parquet.to_str().unwrap());
    }
}
