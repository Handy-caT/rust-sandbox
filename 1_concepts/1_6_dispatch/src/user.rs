use std::borrow::Cow;

#[derive(Debug)]
pub struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

impl User {
    pub fn new(id: u64, email: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id,
            email: email.into(),
            activated: false,
        }
    }

    pub fn activate(&mut self) {
        self.activated = true;
    }

    pub fn deactivate(&mut self) {
        self.activated = false;
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            email: self.email.clone(),
            activated: self.activated,
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.email == other.email && self.activated == other.activated
    }
}

#[cfg(test)]
mod tests {
    use crate::user::User;

    #[test]
    fn test_user() {
        let user = User::new(1, "test@gmail.com");
        assert_eq!(user.get_id(), 1);
        assert_eq!(user.get_email(), "test@gmail.com");
        assert!(!user.is_activated());
    }
}