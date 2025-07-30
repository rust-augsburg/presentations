//! This module contains some crazy business logic

use std::collections::HashMap;

use anyhow::bail;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

pub struct NewUser {
    name: String,
}

pub struct ReadUser<'a> {
    name: &'a str,
}

pub struct UserManager {
    pub storage: HashMap<Uuid, User>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// Add user with random chance of failure :-)
    pub fn create(&mut self, new_user: NewUser) -> anyhow::Result<Uuid> {
        let success = rand::random_bool(0.8);
        if !success {
            bail!("Error, lost connection to database or something");
        }

        let user = User::with_auto_id(new_user);
        let id = user.id;
        self.storage.insert(user.id, user);

        Ok(id)
    }

    pub fn read_by_name(&mut self, user: ReadUser) -> anyhow::Result<Option<User>> {
        let success = rand::random_bool(0.5);
        if !success {
            bail!("Read error, lost connection to database or something");
        }

        let name = user.name;
        let users: Vec<_> = self
            .storage
            .iter()
            .filter(|(_id, user)| user.name.as_str() == name)
            .collect();

        if users.len() > 1 {
            bail!("corrupt database - more than one user with name {name} in it")
        }

        Ok(users.first().map(|(_id, user)| (*user).clone()))
    }
}

impl User {
    fn with_auto_id(new_user: NewUser) -> Self {
        User {
            id: Uuid::new_v4(),
            name: new_user.name,
        }
    }
}

impl NewUser {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

impl<'a> ReadUser<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}
