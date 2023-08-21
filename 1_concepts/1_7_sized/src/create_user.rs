use step_1_6::base::UserRepository;
use step_1_6::user::User;
use crate::command::Command;

pub struct CreateUser {
}

impl CreateUser {
    pub fn new() -> Self {
        Self{}
    }
}

impl Command for CreateUser {
    type Context = dyn UserRepository;

    fn execute(&self, ctx: &mut Self::Context, user: &User) {
        ctx.add(user);
    }
}