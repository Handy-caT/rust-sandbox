use std::collections::HashSet;
use im::{HashMap, OrdMap, Vector};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Id(pub u32);

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct User {
    id: Id,
    nickname: String,
}

trait UserRepository {
    fn get_user_by_id(&self, id: Id) -> Option<User>;
    fn get_users_by_ids(&self, ids: &[Id]) -> std::collections::HashMap<Id, User>;
    fn search_contain_nickname<S: AsRef<str>>(&self, nickname: S) -> HashSet<User>;

}

struct VecUserRepository {
    users: HashMap<Id, User>,
}

impl VecUserRepository {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
}

impl UserRepository for VecUserRepository {
    fn get_user_by_id(&self, id: Id) -> Option<User> {
        self.users.get(&id).cloned()
    }

    fn get_users_by_ids(&self, ids: &[Id]) -> std::collections::HashMap<Id, User> {
        let mut found_users = std::collections::HashMap::new();

        for id in ids {
            if let Some(user) = self.get_user_by_id(*id) {
                found_users.insert(*id, user.clone());
            }
        }

        found_users
    }

    fn search_contain_nickname<S: AsRef<str>>(&self, nickname: S) -> HashSet<User> {
        let mut found_users = HashSet::new();
        let nickname = nickname.as_ref();
        for user in self.users.values() {
            if user.nickname.contains(nickname) {
                found_users.insert(user.clone());
            }
        }

        found_users
    }
}

fn main() {
    println!("Implement me!");
}


#[cfg(test)]
mod tests {
    use crate::{Id, User, UserRepository, VecUserRepository};

    #[test]
    fn test_user_repo_new() {
        let repo = VecUserRepository::new();
        assert_eq!(repo.users.len(), 0);
    }

    #[test]
    fn test_user_repo_get_by_id_exists() {
        let mut repo = VecUserRepository::new();
        let user = User {
            id: Id(1),
            nickname: "test".to_string(),
        };
        repo.add_user(user.clone());
        assert_eq!(repo.get_user_by_id(Id(1)), Some(user));
    }

    #[test]
    fn test_user_repo_get_by_id_not_exists() {
        let repo = VecUserRepository::new();
        assert_eq!(repo.get_user_by_id(Id(1)), None);
    }

    #[test]
    fn test_user_repo_get_by_ids_exist() {
        let mut repo = VecUserRepository::new();
        let user1 = User {
            id: Id(1),
            nickname: "test1".to_string(),
        };
        let user2 = User {
            id: Id(2),
            nickname: "test2".to_string(),
        };
        repo.add_user(user1.clone());
        repo.add_user(user2.clone());
        let users = repo.get_users_by_ids(vec![Id(1), Id(2)].as_slice());
        assert_eq!(users.len(), 2);
        assert_eq!(users.get(&Id(1)), Some(&user1));
        assert_eq!(users.get(&Id(2)), Some(&user2));
    }

    #[test]
    fn test_user_repo_get_by_ids_not_exist() {
        let mut repo = VecUserRepository::new();
        let user1 = User {
            id: Id(1),
            nickname: "test1".to_string(),
        };
        let user2 = User {
            id: Id(2),
            nickname: "test2".to_string(),
        };
        repo.add_user(user1.clone());
        let users = repo.get_users_by_ids(vec![Id(1), Id(2)].as_slice());
        assert_eq!(users.len(), 1);
        assert_eq!(users.get(&Id(1)), Some(&user1));
        assert_eq!(users.get(&Id(2)), None);
    }

    #[test]
    fn test_user_repo_search_contain_nickname_exist() {
        let mut repo = VecUserRepository::new();
        let user1 = User {
            id: Id(1),
            nickname: "test1".to_string(),
        };
        let user2 = User {
            id: Id(2),
            nickname: "test2".to_string(),
        };
        repo.add_user(user1.clone());
        repo.add_user(user2.clone());

        let result = repo.search_contain_nickname("test".to_string());
        assert_eq!(result.len(), 2);
        assert!(result.contains(&user1));
        assert!(result.contains(&user2));
    }

    #[test]
    fn test_user_repo_search_contain_nickname_not_exist() {
        let mut repo = VecUserRepository::new();
        let user1 = User {
            id: Id(1),
            nickname: "test1".to_string(),
        };
        let user2 = User {
            id: Id(2),
            nickname: "test2".to_string(),
        };
        repo.add_user(user1.clone());
        repo.add_user(user2.clone());

        let result = repo.search_contain_nickname("test3".to_string());
        assert!(result.is_empty());
    }
}