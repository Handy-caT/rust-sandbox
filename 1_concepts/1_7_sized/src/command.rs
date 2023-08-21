use step_1_6::user::User;

pub trait Command {
    type Context: ?Sized;
    fn execute(&self, ctx: &Self::Context, user: &User);
}