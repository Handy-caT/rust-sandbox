use crate::post::Post;

pub struct New();
pub struct Unmoderated();
pub struct Published();
pub struct Deleted();

pub enum PostWrapper {
    New(Post<New>),
    Unmoderated(Post<Unmoderated>),
    Published(Post<Published>),
    Deleted(Post<Deleted>),
}

pub struct PostFactory {
    post: PostWrapper
}

impl PostFactory {
    pub fn new() -> Post<New> {

    }

}