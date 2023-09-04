use crate::user;
use crate::wrapper::{Deleted, New, Published, Unmoderated};


#[derive(Clone, Debug, PartialEq)]
pub struct Id(pub u64);

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Title(pub String);

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Body(pub String);


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
            title: Title::default(),
            body: Body::default(),
            status: New(),
        }
    }

    pub fn publish(self) -> Post<Unmoderated> {
        self.into()
    }
}

impl Post<New> {
    pub fn set_title(&mut self, title: Title) {
        self.title = title;
    }

    pub fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

impl Post<Unmoderated> {
    pub fn deny(self) -> Post<Deleted> {
        self.into()
    }

    pub fn allow(self) -> Post<Published> {
        self.into()
    }
}

impl Post<Published> {
    fn delete(self) -> Post<Deleted> {
        self.into()
    }
}

impl From<Post<New>> for Post<Unmoderated> {
    fn from(post: Post<New>) -> Post<Unmoderated> {
        Post {
            id: post.id,
            user_id: post.user_id,
            title: post.title,
            body: post.body,
            status: Unmoderated(),
        }
    }
}

impl From<Post<Unmoderated>> for Post<Deleted> {
    fn from(post: Post<Unmoderated>) -> Post<Deleted> {
        Post {
            id: post.id,
            user_id: post.user_id,
            title: post.title,
            body: post.body,
            status: Deleted(),
        }
    }
}

impl From<Post<Unmoderated>> for Post<Published> {
    fn from(post: Post<Unmoderated>) -> Post<Published> {
        Post {
            id: post.id,
            user_id: post.user_id,
            title: post.title,
            body: post.body,
            status: Published(),
        }
    }
}

impl From<Post<Published>> for Post<Deleted> {
    fn from(post: Post<Published>) -> Post<Deleted> {
        Post {
            id: post.id,
            user_id: post.user_id,
            title: post.title,
            body: post.body,
            status: Deleted(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::post::{Id, Post};
    use crate::user;
    use crate::wrapper::{Deleted, New, Published, Unmoderated};

    #[test]
    fn test_post_actions_deny() {
        let id = Id(1);
        let user_id = user::Id(1);

        let post = Post::new(id, user_id);
        assert_eq!(post.status, New());

        let published_post = post.publish();
        assert_eq!(published_post.status, Unmoderated());

        let denied_post = published_post.deny();
        assert_eq!(denied_post.status, Deleted());
    }

    #[test]
    fn test_post_actions_allow() {
        let id = Id(1);
        let user_id = user::Id(1);

        let post = Post::new(id, user_id);
        assert_eq!(post.status, New());

        let published_post = post.publish();
        assert_eq!(published_post.status, Unmoderated());

        let allowed_post = published_post.allow();
        assert_eq!(allowed_post.status, Published());

        let deleted_post = allowed_post.delete();
        assert_eq!(deleted_post.status, Deleted());
    }
}