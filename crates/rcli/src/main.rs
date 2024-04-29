use clap::Parser;

/// Show CSV, or convert CSV to other formats
#[derive(Parser, Debug)]
struct CsvArgs {
    /// Input file
    #[arg(long, value_parser = verify_input_file)]
    input: String,

    /// Output file
    #[arg(long, default_value = "output.json")]
    output: String,

    /// Delimiter
    #[arg(long, default_value_t = ',')]
    delimiter: char,

    /// Whether the input file has a header
    #[arg(long, default_value_t = true)]
    header: bool,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(name = "csv")]
    Csv(CsvArgs),
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: SubCommand,
}

// rcli csv -i input.csv -o output.json --header -d ','

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file does not exist")
    }
}