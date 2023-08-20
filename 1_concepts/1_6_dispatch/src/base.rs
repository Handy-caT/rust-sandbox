use crate::user::User;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}


pub trait UserRepository {
    fn add(user: User);
    fn get(id: u64) -> Option<User>;
    fn remove(id: u64) -> Option<User>;
    fn update(id: u64, user: User);
}