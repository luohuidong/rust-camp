// rcli csv -i input.csv -o output.json --header -d ','

use anyhow;
use clap::Parser;
use rcli::process_csv;

mod opts;

fn main() -> anyhow::Result<()> {
    let args = opts::Opts::parse();

    match args.cmd {
        opts::SubCommand::Csv(args) => process_csv(&args.input, &args.output)?,
    }

    anyhow::Result::Ok(())
}
