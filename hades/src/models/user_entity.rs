

use poem_openapi::Object;
use uuid::Timestamp;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Clone, serde::Deserialize, Object)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Deserialize, Object)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>, password: impl Into<String>) -> Self {
        let id: uuid::Uuid = uuid::Uuid::parse_str(&id.into())
            .unwrap_or_else(|_| uuid::Uuid::new_v7(Timestamp::now(uuid::timestamp::context::ContextV7::new())));
        let password: String = bcrypt::hash(password.into(),8).unwrap();
        let email: String = email.into();
        let name: String = name.into();
        Self { id, name, email, password }
    }
    pub fn new_from_register(register: Register) -> Self {
        let id: uuid::Uuid = uuid::Uuid::new_v7(Timestamp::now(uuid::timestamp::context::ContextV7::new()));
        let password: String = bcrypt::hash(register.password,8).unwrap();
        let email: String = register.email;
        let name: String = register.name;
        Self { id, name, email, password }
    }
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
    
}