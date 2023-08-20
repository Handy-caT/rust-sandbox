use crate::base::Storage;
use crate::user::User;

struct UserRepository {
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepository {
    fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self { storage }
    }
}

impl UserRepository {
    fn add(&mut self, user: User) {
        self.storage.set(user.get_id(), user);
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
    use crate::dynamic::UserRepository;
    use crate::storage::HashTableStorage;
    use crate::user::User;

    #[test]
    fn test_user_repository() {
        let mut user_repository = UserRepository::new(Box::new(HashTableStorage::new()));
        let user = User::new(1, "test@gmail.com");

        user_repository.add(user.clone());

        assert_eq!(user_repository.get(1), Some(&user));
    }
}