use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "GitLab CLI", version, about = "Command line ineterface for GitLab", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get entity
    Get {
        #[command(subcommand)]
        command: GetAction,
    },
}

#[derive(Subcommand)]
enum GetAction {
    /// User entity
    #[command(alias="u")]
    User(GetUserArgs),
}

#[derive(Args)]
struct GetUserArgs {
    username: String,
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { command } => match command {
            GetAction::User(username) => {
                println!("Performing action with option: {}", username.username);
            },
        }
    }
}