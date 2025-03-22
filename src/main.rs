use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "GitLab Group CLI", version, about = "Command line ineterface for GitLab group management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage GitLab groups
    #[command(alias="g")]
    Group {
        /// Group name
        groupname: String,

        #[command(subcommand)]
        command: GroupAction,
    },
    /// Manage GitLab users
    #[command(alias="u")]
    User {
        /// Get target user
        username: String,
    },
}

#[derive(Subcommand)]
enum GroupAction {
    /// List group members
    #[command(alias="l")]
    List,

    /// Append group members with user
    Append {
        username: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Group { groupname, command } => match command {
            GroupAction::List => {
                println!("Action [LIST] with group {groupname}");
            },
            GroupAction::Append { username } => {
                println!("Action [APPEND] with group {groupname}, and user {username}");
            },
        },
        Commands::User { username } => {
            println!("Action [GET] with user {username}");
        },
    }
}