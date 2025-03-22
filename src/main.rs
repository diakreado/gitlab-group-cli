mod gitlab_api;
mod app_cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app_cli::parse();

    match gitlab_api::fetch_user("diakreado".to_string()) {
        Ok(user) => println!("User details: {}", user),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}