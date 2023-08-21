use std::fmt::Error;
use step_1_6::base::UserRepository;
use step_1_6::user::User;
use crate::command::Command;
use crate::command_handler::CommandHandler;
use crate::create_user::CreateUser;

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), Error>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        cmd.execute(ctx, self);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use step_1_6::base::UserRepository;
    use step_1_6::r#static::StaticUserRepository;
    use step_1_6::storage::HashTableStorage;
    use step_1_6::user::User;
    use crate::command_handler::CommandHandler;
    use crate::create_user::CreateUser;

    #[test]
    fn test_user_repository() {
        let mut user_repository = StaticUserRepository::new(HashTableStorage::new());
        let user = User::new(1, "test@gmail.com");

        let create_user = CreateUser::new();
        user.handle_command(&create_user, &mut user_repository).unwrap();

        assert_eq!(user_repository.get(1), Some(&user));
    }
}