use crate::base::{Storage, UserRepository};
use crate::user::User;

pub struct StaticUserRepository<S> {
    storage: S,
}

impl<S> StaticUserRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S> UserRepository for StaticUserRepository<S>
where
    S: Storage<u64, User>,
{
    fn add(&mut self, user: &User) {
        self.storage.set(user.get_id(), user.clone());
    }

    fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }

    fn update(&mut self, id: u64, user: User) {
        self.storage.set(id, user);
    }
}

#[cfg(test)]
mod tests {
    use crate::r#static::{StaticUserRepository, UserRepository};
    use crate::storage::HashTableStorage;
    use crate::user::User;

    #[test]
    fn test_user_repository() {
        let mut user_repository = StaticUserRepository::new(HashTableStorage::new());
        let user = User::new(1, "test@gmail.com");
        user_repository.add(&user);

        assert_eq!(user_repository.get(1), Some(&user));
    }
}
