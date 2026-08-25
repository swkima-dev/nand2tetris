use anyhow::Result;
use clap::Parser;
use hack_assembler::{self, hack::assembler};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let code = assembler(&args.path);

    println!("{}", code.unwrap());

    Ok(())
}
