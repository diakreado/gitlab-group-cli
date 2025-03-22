use clap::{Parser, Subcommand};

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
    User {
        username: String,
    },
    /// Group entity
    #[command(alias="g")]
    Group {
        groupname: String
    },
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { command } => match command {
            GetAction::User { username } => {
                println!("Action with user: {username}");
            },
            GetAction::Group { groupname } => {
                println!("Action with group: {groupname}");
            },
        }
    }
}