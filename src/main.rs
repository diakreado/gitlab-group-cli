use clap::Parser;

#[derive(Parser)]
#[command(name = "GitLab CLI")]
#[command(version, about = "Command line ineterface for GitLab", long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.count {
        println!("Hello {}!", cli.name);
    }
}