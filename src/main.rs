use reqwest;
use text_io::*;
use tokio;
mod models;
mod controller;
use crate::models::User; 
use crate::models::NewUser; 
use crate::controller::login;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Insert your email: ");
    let email: String = read!();
    println!("Insert your password: ");
    let password: String = read!();
    let user = User::new(email.to_string(), password.to_string());

    login(&user).await?;

    Ok(())
}


