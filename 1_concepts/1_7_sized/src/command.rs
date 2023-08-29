use step_1_6::user::User;

pub trait Command {
    type Context: ?Sized;
    fn execute(&self, ctx: &mut Self::Context, user: &User);
}