use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use entities::user;
use crate::cli_processor::CliCommand;

pub struct DBProcessor {
    db: DatabaseConnection
}

impl DBProcessor {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self {
            db: connection
        }
    }

    pub async fn process_command(&self, command: CliCommand) {
        match command {
            CliCommand::AddUser(user, mut user_role) => {
                let user = user.insert(&self.db).await.unwrap();
                user_role.user_id = Set(user.id);
                user_role.insert(&self.db).await.unwrap();
            }
            CliCommand::AddRole(role) => {
                role.insert(&self.db).await.unwrap();
            }
            CliCommand::DeleteUserName(name) => {
                let user = user::Entity::find()
                    .filter(user::Column::Name.contains(name))
                    .one(&self.db)
                    .await
                    .unwrap()
                    .unwrap();
                user.delete(&self.db).await.unwrap();
            }
            CliCommand::DeleteUserId(id) => {
                let user = user::Entity::find_by_id(id)
                    .one(&self.db)
                    .await
                    .unwrap()
                    .unwrap();
                user.delete(&self.db).await.unwrap();
            }
            CliCommand::DeleteRoleSlug(slug) => {
                let role = entities::role::Entity::find()
                    .filter(entities::role::Column::Slug.contains(slug))
                    .one(&self.db)
                    .await
                    .unwrap()
                    .unwrap();
                role.delete(&self.db).await.unwrap();
            }
        }
    }

}