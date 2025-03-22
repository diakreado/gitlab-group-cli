use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    Id: u64,
    name: String,
    username: String,
    state: String,
    locked: bool,
}

pub fn fetch_user(username: String) -> Result<User, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .get(format!("https://gitlab.com/api/v4/users?username{username}"))
        .header("PRIVATE-TOKEN", "TOKEN")
        .send()?;

    if response.status().is_success() {
        let user: User = response.json()?;
        Ok(user)
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}
