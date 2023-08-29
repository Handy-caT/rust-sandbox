use crate::user::User;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}


pub trait UserRepository {
    fn add(&mut self, user: &User);
    fn get(&self, id: u64) -> Option<&User>;
    fn remove(&mut self, id: u64) -> Option<User>;
    fn update(&mut self, id: u64, user: User);
}