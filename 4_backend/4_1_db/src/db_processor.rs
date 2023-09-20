use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, JoinType, ModelTrait, QueryFilter, QuerySelect, QueryTrait, RelationDef, Statement};
use sea_orm::ActiveValue::Set;
use entities::{role, user, users_roles};
use entities::user::Model;
use crate::cli_processor::CliCommand;


#[derive(FromQueryResult, Debug)]
struct UserAndRole {
    id: i32,
    name: String,
    role_slug: String,
}

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
            CliCommand::DeleteUserId(id) => {
                let user = user::Entity::find_by_id(id.0)
                    .one(&self.db)
                    .await
                    .unwrap()
                    .unwrap();
                user.delete(&self.db).await.unwrap();
            }
            CliCommand::DeleteRoleSlug(slug) => {
                let role = entities::role::Entity::find()
                    .filter(entities::role::Column::Slug.contains(slug.0))
                    .one(&self.db)
                    .await
                    .unwrap()
                    .unwrap();
                role.delete(&self.db).await.unwrap();
            }
            CliCommand::UpdateUser(id, name) => {
                let user = user::Entity::find_by_id(id.0)
                    .one(&self.db)
                    .await
                    .unwrap();

                let mut user: user::ActiveModel = user.unwrap().into();
                user.name = Set(name.0);

                user.update(&self.db).await.unwrap();
            }
            CliCommand::UpdateRole(slug, name, permissions) => {
                let role = entities::role::Entity::find()
                    .filter(entities::role::Column::Slug.eq(slug.0))
                    .one(&self.db)
                    .await
                    .unwrap();

                let mut role: entities::role::ActiveModel = role.unwrap().into();

                if let Some(name) = name {
                    role.name = Set(name.0);
                }

                if let Some(permissions) = permissions {
                    role.permissions = Set(permissions.0);
                }

                role.update(&self.db).await.unwrap();
            }
            CliCommand::AssignRole(user_id, role_slug) => {
                let role = role::Entity::find()
                    .filter(role::Column::Slug.eq(&role_slug.0))
                    .one(&self.db)
                    .await
                    .unwrap();

                if role.is_some() {
                    let user_role = users_roles::Entity::find()
                        .filter(users_roles::Column::UserId.eq(user_id.0))
                        .filter(users_roles::Column::RoleSlug.eq(&role_slug.0))
                        .one(&self.db)
                        .await
                        .unwrap();

                    if user_role.is_none() {
                        let user_role = users_roles::ActiveModel {
                            user_id: Set(user_id.0),
                            role_slug: Set(role_slug.0),
                            ..Default::default()
                        };

                        user_role.insert(&self.db).await.unwrap();
                    }
                }
                else {
                    println!("Role with slug {} does not exist", role_slug.0 )
                }
            }
            CliCommand::UnassignRole(user_id, role_slug) => {
                let user_role = users_roles::Entity::find()
                    .filter(users_roles::Column::UserId.eq(user_id.0))
                    .filter(users_roles::Column::RoleSlug.eq(&role_slug.0))
                    .one(&self.db)
                    .await
                    .unwrap();

                if let Some(user_role) = user_role {
                    user_role.delete(&self.db).await.unwrap();
                }
            }
            CliCommand::ShowRoles => {
                let roles = entities::role::Entity::find()
                    .all(&self.db)
                    .await
                    .unwrap();

                for role in roles {
                    println!("{:?}", role);
                }
            }
            CliCommand::ShowRole(slug) => {
                let role = entities::role::Entity::find()
                    .filter(entities::role::Column::Slug.eq(slug.0))
                    .one(&self.db)
                    .await
                    .unwrap();

                println!("{:?}", role);
            }
            CliCommand::ShowUser(id) => {
                let user_select: Vec<UserAndRole> = user::Entity::find_by_id(id.0)
                    .column(users_roles::Column::RoleSlug)
                    .join_rev(
                        JoinType::LeftJoin,
                        users_roles::Entity::belongs_to(user::Entity)
                            .from(users_roles::Column::UserId)
                            .to(user::Column::Id)
                            .into()
                    )
                    .into_model::<UserAndRole>()
                    .all(&self.db)
                    .await
                    .unwrap();

                for user in user_select {
                    println!("{:?}", user);
                }
            }
            CliCommand::ShowUsers => {
                let user_select: Vec<UserAndRole> = user::Entity::find()
                    .column(users_roles::Column::RoleSlug)
                    .join_rev(
                        JoinType::LeftJoin,
                        users_roles::Entity::belongs_to(user::Entity)
                            .from(users_roles::Column::UserId)
                            .to(user::Column::Id)
                            .into()
                    )
                    .into_model::<UserAndRole>()
                    .all(&self.db)
                    .await
                    .unwrap();

                for user in user_select {
                    println!("{:?}", user);
                }
            }
        }
    }

}