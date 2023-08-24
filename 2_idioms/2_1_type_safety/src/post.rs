use crate::user;
use crate::wrapper::New;


#[derive(Clone, Debug, PartialEq)]
pub struct Id(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Title(String);
impl Title {
    pub fn new() -> Title {
        Title(String::new())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Body(String);
impl Body {
    pub fn new() -> Body {
        Body(String::new())
    }
}


#[derive(Clone)]
pub struct Post<Status> {
    id: Id,
    user_id: user::Id,
    title: Title,
    body: Body,
    status: Status
}

impl Post<New> {
    pub fn new(id: Id, user_id: user::Id) -> Post<New> {
        Post {
            id,
            user_id,
            title: Title::new(),
            body: Body::new(),
            status: New(),
        }
    }
}
