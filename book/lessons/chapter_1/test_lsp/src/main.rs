use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: Option<String>,
}

impl User {
    fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            email: None,
        }
    }

    fn set_email(&mut self, email: impl Into<String>) {
        self.email = Some(email.into());
    }
}

trait Identifiable {
    fn id(&self) -> u32;
}

impl Identifiable for User {
    fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
enum AppError {
    NotFound(u32),
    InvalidEmail(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(id) => write!(f, "User with id {} not found", id),
            AppError::InvalidEmail(email) => write!(f, "Invalid email: {}", email),
        }
    }
}

impl std::error::Error for AppError {}

fn validate_email(email: &str) -> Result<(), AppError> {
    if email.contains('@') {
        Ok(())
    } else {
        Err(AppError::InvalidEmail(email.to_string()))
    }
}

fn find_by_id<T: Identifiable>(items: &[T], id: u32) -> Result<&T, AppError> {
    items
        .iter()
        .find(|item| item.id() == id)
        .ok_or(AppError::NotFound(id))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut users = Vec::new();

    let mut alice = User::new(1, "Alice");
    alice.set_email("alice@example.com");

    let mut bob = User::new(2, "Bob");
    bob.set_email("1@");

    users.push(alice);
    users.push(bob);

    for user in &mut users {
        if let Some(email) = &user.email {
            validate_email(email)?;
        }
        let name = &user.name;
        println!("{name}")
    }

    let found = find_by_id(&users, 1)?;
    println!("Found user: {:?}", found);

    // Intentional unused variable to trigger warnings, remove _ to trigger warning
    let _unused_map: HashMap<u32, String> = HashMap::new();

    Ok(())
}
