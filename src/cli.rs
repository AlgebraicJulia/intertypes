use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    input_file: String,
}

pub fn run() {
    let args = Args::parse();

    println!("{}", args.input_file);
}
