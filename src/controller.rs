use crate::reqwest::header::{HeaderValue, AUTHORIZATION};
use crate::NewUser;
use crate::User;
use text_io::*;

pub async fn login(user: &User) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:8000/users";
    let token = format!("{}:{}", user.email.clone(), user.password.clone());
    let encoded_token = base64::encode(token);
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded_token))?,
        )
        .send()
        .await?;

    if response.status().is_success() {
        println!("\nSuccessful login!\n");
        menu_operations(User::new(user.email.clone(), user.password.clone())).await?;
    } else {
        println!("Failed to fetch data: {}", response.status());
        println!("Account not found, create one? y/n");
        let res: String = read!();
        if res == "y" || res == "Y" || res == "yes" || res == "YES" {
            create_account().await?;
        } else {
            println!("Program finished");
        }
    }
    Ok(())
}


async fn create_account() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:8000/users";

    println!("Insert username: ");
    let name: String = read!();
    println!("Insert email: ");
    let email: String = read!();
    println!("Insert password: ");
    let password: String = read!();

    let new_user = NewUser::new(name, email.clone(), password.clone());

    let token = format!("{}:{}", new_user.email, new_user.password);
    let encoded_token = base64::encode(token);

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(&new_user)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded_token))?,
        )
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let pretty_json = serde_json::to_string_pretty(&json)?;
        println!("Response Body:\n{}", pretty_json);
        menu_operations(User::new(email, password)).await?;
    } else {
        println!("Failed to fetch data: {}", response.status());
    }
    Ok(())
}

pub async fn menu_operations(user: User) -> Result<(), Box<dyn std::error::Error>> {
    println!("(1) - Get all users\n(2) - Get user by id\n(3) - Create user\n(4) - Update user information\n(5) - Delete user\n(6) - Leave\n");
    let mut res: i32 = read!();

    while res != 6 {
        match res {
            1 => {
                get_all(&user.email, &user.password).await?;
            }
            2 => {
                get_by_id(&user.email, &user.password).await?;
            }
            3 => {
                create_user().await?;
            }
            4 => {
                update_user(&user).await?;
            }
            5 => {
                delete_user(&user.email, &user.password).await?;
            }
            6 => {
                break;
            }
            _ => {
                println!("Invalid operation")
            }
        }
        println!("\n(1) - Get all users\n(2) - Get user by id\n(3) - Create user\n(4) - Update user information\n(5) - Delete user\n(6) - Leave\n");
        res = read!();
    }

    println!("Process exited with success");

    Ok(())
}

async fn get_all(email: &String, password: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:8000/users";

    let token = format!("{}:{}", email, password);
    let encoded_token = base64::encode(token);

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded_token))?,
        )
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let pretty_json = serde_json::to_string_pretty(&json)?;
        println!("Response Body:\n{}", pretty_json);
    } else {
        println!("Failed to fetch data: {}", response.status());
    }
    Ok(())
}

async fn get_by_id(email: &String, password: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Insert user id");
    let id: i32 = read!();
    let url = format! {"http://localhost:8000/users/{}", id};

    let token = format!("{}:{}", email, password);
    let encoded_token = base64::encode(token);

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded_token))?,
        )
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let pretty_json = serde_json::to_string_pretty(&json)?;
        println!("Response Body:\n{}", pretty_json);
    } else {
        println!("Failed to fetch data: {}", response.status());
    }
    Ok(())
}

async fn create_user() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:8000/users";

    println!("Insert username: ");
    let name: String = read!();
    println!("Insert email: ");
    let email: String = read!();
    println!("Insert password: ");
    let password: String = read!();

    let new_user = NewUser::new(name, email.clone(), password.clone());

    let token = format!("{}:{}", new_user.email, new_user.password);
    let encoded_token = base64::encode(token);

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(&new_user)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded_token))?,
        )
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let pretty_json = serde_json::to_string_pretty(&json)?;
        println!("Response Body:\n{}", pretty_json);
    } else {
        println!("Failed to fetch data: {}", response.status());
    }
    Ok(())
}

async fn update_user(current_user: &User) -> Result<(), Box<dyn std::error::Error>> {
    println!("Insert user id: ");
    let id: i32 = read!();
    let url = format!("http://localhost:8000/users/{}", id);

    println!("Insert username: ");
    let name: String = read!("{}\n");
    println!("Insert email: ");
    let email: String = read!();
    println!("Insert password: ");
    let password: String = read!();

    let new_user = NewUser::new(name, email.clone(), password.clone());

    let token = format!("{}:{}", current_user.email, current_user.password);
    let encoded_token = base64::encode(token);

    let client = reqwest::Client::new();

    let response = client
        .put(url)
        .json(&new_user)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded_token))?,
        )
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let pretty_json = serde_json::to_string_pretty(&json)?;
        println!("Response Body:\n{}", pretty_json);
    } else {
        println!("Failed to fetch data: {}", response.status());
    }
    Ok(())
}
async fn delete_user(email: &String, password: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Insert user id: ");
    let id: i32 = read!();
    let url = format!("http://localhost:8000/users/{}", id);

    let token = format!("{}:{}", email, password);
    let encoded_token = base64::encode(token);

    let client = reqwest::Client::new();

    let response = client
        .delete(url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded_token))?,
        )
        .send()
        .await?;

    if response.status().is_success() {
        println!("User deleted with success!");
    } else {
        println!("Failed to fetch data: {}", response.status());
    }
    Ok(())
}
