use im::{OrdMap, Vector};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct Id(pub u32);

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct User {
    id: Id,
    nickname: String,
}

trait UserRepository {
    fn get_user_by_id(&self, id: Id) -> Option<&User>;
    fn get_users_by_ids(&self, ids: Vec<Id>) -> Vec<&User>;
    fn search_contain_nickname(&self, nickname: String) -> Vec<&User>;

}

struct VecUserRepository {
    users: OrdMap<Id, User>,
}

impl

impl UserRepository for VecUserRepository {
    fn get_user_by_id(&self, id: Id) -> Option<&User> {
        self.users.get(&id)
    }

    fn get_users_by_ids(&self, ids: Vec<Id>) -> Vec<&User> {
        unimplemented!()
    }

    fn search_contain_nickname(&self, nickname: String) -> Vec<&User> {
        unimplemented!()
    }
}

fn main() {
    println!("Implement me!");
}
