use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Cmd,
}

#[derive(Parser)]
enum Cmd {
    Tui,
}

fn main() {
    let args = Args::parse();
}
