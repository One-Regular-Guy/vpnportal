

use poem_openapi::Object;
use uuid::Timestamp;

#[derive(Debug, Clone)]
pub struct User {
    id: uuid::Uuid,
    name: String,
    email: String,
    password: String
}

#[derive(Debug, Clone, serde::Deserialize, Object)]
pub struct Login {
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