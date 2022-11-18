use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 500)]
    milliseconds: u64,
}

fn main() {
    let args = Args::parse();

    let mut siv = usage::create_cursive_runnable(args.milliseconds);
    siv.run();
}
