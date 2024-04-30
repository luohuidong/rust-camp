use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv")]
    Csv(CsvOpts),
}

/// Show CSV, or convert CSV to other formats
#[derive(Parser, Debug)]
pub struct CsvOpts {
    /// Input file
    #[arg(long, value_parser = verify_input_file)]
    pub input: String,

    /// Output file
    #[arg(long, default_value = "output.json")]
    pub output: String,

    /// Delimiter
    #[arg(long, default_value_t = ',')]
    pub delimiter: char,

    /// Whether the input file has a header
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file does not exist")
    }
}
