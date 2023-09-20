use actix_web::{Error, error};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, ModelTrait, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use entities::{role, user, users_roles};

#[derive(FromQueryResult, Debug, Serialize, Deserialize)]
struct UserAndRole {
    id: i32,
    name: String,
    role_slug: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    AddUser,
    AddRole,
    DeleteUser,
    DeleteRole,
    UpdateUser,
    UpdateRole,
    AssignRole,
    UnassignRole,
    ShowUsers,
    ShowRoles,
    ShowUser,
    ShowRole,
}

pub struct RequestProcessor {
    db: DatabaseConnection
}

impl RequestProcessor {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self {
            db: db_conn
        }
    }

    fn get_command_type(json: &Value) -> Result<RequestType, Error> {
        let type_string = Self::unwrap_json_string(json, "command_type")?;

        match type_string.as_str() {
            "add_user" => Ok(RequestType::AddUser),
            "add_role" => Ok(RequestType::AddRole),
            "delete_user" => Ok(RequestType::DeleteUser),
            "delete_role" => Ok(RequestType::DeleteRole),
            "update_user" => Ok(RequestType::UpdateUser),
            "update_role" => Ok(RequestType::UpdateRole),
            "assign_role" => Ok(RequestType::AssignRole),
            "unassign_role" => Ok(RequestType::UnassignRole),
            "show_users" => Ok(RequestType::ShowUsers),
            "show_roles" => Ok(RequestType::ShowRoles),
            "show_user" => Ok(RequestType::ShowUser),
            "show_role" => Ok(RequestType::ShowRole),
            _ => Err(error::ErrorBadRequest("unknown command type"))
        }
    }

    fn unwrap_json_string<S: AsRef<str>>(json: &Value, key: S) -> Result<String, Error> {
        let value = json.get(key.as_ref());
        if value.is_none() {
            return Err(error::ErrorBadRequest(format!("{} is missing", key.as_ref())));
        }
        if !value.unwrap().is_string() {
            return Err(error::ErrorBadRequest(format!("{} is not a string", key.as_ref())));
        }
        let value = value.unwrap().as_str();
        if value.is_none() {
            return Err(error::ErrorBadRequest(format!("{} is not a string", key.as_ref())));
        }
        Ok(value.unwrap().to_owned())
    }

    fn unwrap_json_i32<S: AsRef<str>>(json: &Value, key: S) -> Result<i32, Error> {
        let value = json.get(key.as_ref());
        if value.is_none() {
            return Err(error::ErrorBadRequest(format!("{} is missing", key.as_ref())));
        }
        if !value.unwrap().is_number() {
            return Err(error::ErrorBadRequest(format!("{} is not a number", key.as_ref())));
        }
        let value = value.unwrap().as_i64();
        if value.is_none() {
            return Err(error::ErrorBadRequest(format!("{} is not a number", key.as_ref())));
        }
        let value = value.unwrap();
        let value = value.try_into();
        if value.is_err() {
            return Err(error::ErrorBadRequest(format!("{} is not a number", key.as_ref())));
        }
        Ok(value.unwrap())
    }

    async fn process_user_add(&self, json: &Value) -> Result<String, Error> {
        let name = Self::unwrap_json_string(json, "name")?;
        let role_slug = Self::unwrap_json_string(json, "role")?;

        let user = user::ActiveModel {
            name: Set(name),
            ..Default::default()
        };

        let mut user_role = users_roles::ActiveModel {
            role_slug: Set(role_slug),
            ..Default::default()
        };

        let user = user.insert(&self.db).await;
        if user.is_err() {
            return Err(error::ErrorBadRequest("user already exists"));
        }
        let user = user.unwrap();
        user_role.user_id = Set(user.id);
        let res = user_role.insert(&self.db).await;
        if res.is_err() {
            return Err(error::ErrorBadRequest("user already has this role"));
        }
        let res = res.unwrap();

        let user = serde_json::to_string(&user);
        if user.is_err() {
            return Err(error::ErrorInternalServerError("cannot serialize user"));
        }
        let user = user.unwrap();
        let user_role = serde_json::to_string(&res);
        if user_role.is_err() {
            return Err(error::ErrorInternalServerError("cannot serialize user role"));
        }
        let user_role = user_role.unwrap();
        Ok(format!("{}, {}", user, user_role))
    }

    async fn process_role_add(&self, json: &Value) -> Result<String, Error> {
        let slug = Self::unwrap_json_string(json, "slug")?;
        let name = json.get("name");
        let permissions = json.get("permissions");

        let mut role = role::ActiveModel {
            slug: Set(slug),
            ..Default::default()
        };

        if name.is_some() {
            let name = name.unwrap();
            if !name.is_string() {
                return Err(error::ErrorBadRequest("name is not a string"));
            }
            let name = name.as_str().unwrap();
            role.name = Set(name.to_owned());
        }

        if permissions.is_some() {
            let permissions = permissions.unwrap();
            if !permissions.is_string() {
                return Err(error::ErrorBadRequest("permissions is not a string"));
            }
            let permissions = permissions.as_str().unwrap();
            role.permissions = Set(permissions.to_owned());
        }

        let role = role.insert(&self.db).await;
        if role.is_err() {
            return Err(error::ErrorBadRequest("role already exists"));
        }
        let role = role.unwrap();


        let role = serde_json::to_string(&role);
        if role.is_err() {
            return Err(error::ErrorInternalServerError("cannot serialize role"));
        }
        let role = role.unwrap();
        Ok(role)
    }

    async fn process_delete_user(&self, json: &Value) -> Result<String, Error> {
        let id = Self::unwrap_json_i32(json, "id")?;

        let user = user::Entity::find_by_id(id)
            .one(&self.db)
            .await;

        if user.is_err() {
            Err(error::ErrorBadRequest("user does not exist"))?;
        }
        let user = user.unwrap();
        if user.is_none() {
            Err(error::ErrorBadRequest("user does not exist"))?;
        }
        let user = user.unwrap();

        let user = user.delete(&self.db).await;
        if user.is_err() {
            Err(error::ErrorInternalServerError("cannot delete user"))?;
        }

        let serialized = serde_json::to_string(&id);
        if serialized.is_err() {
            Err(error::ErrorInternalServerError("cannot serialize user id"))?;
        }
        Ok(serialized.unwrap())
    }

    async fn process_role_delete(&self, json: &Value) -> Result<String, Error> {
        let slug = Self::unwrap_json_string(json, "slug")?;

        let role = role::Entity::find()
            .filter(role::Column::Slug.eq(&slug))
            .one(&self.db)
            .await;

        if role.is_err() {
            Err(error::ErrorBadRequest("role does not exist"))?;
        }
        let role = role.unwrap();
        if role.is_none() {
            Err(error::ErrorBadRequest("role does not exist"))?;
        }
        let role = role.unwrap();

        let role = role.delete(&self.db).await;
        if role.is_err() {
            Err(error::ErrorInternalServerError("cannot delete role"))?;
        }
        let serialized = serde_json::to_string(&slug);
        if serialized.is_err() {
            Err(error::ErrorInternalServerError("cannot serialize role slug"))?;
        }
        Ok(serialized.unwrap())
    }

    async fn process_user_update(&self, json: &Value) -> Result<String, Error> {
        let id = Self::unwrap_json_i32(json, "id")?;
        let name = Self::unwrap_json_string(json, "name")?;

        let user = user::Entity::find_by_id(id)
            .one(&self.db)
            .await;

        if user.is_err() {
            Err(error::ErrorBadRequest("user does not exist"))?;
        }
        let user = user.unwrap();
        if user.is_none() {
            Err(error::ErrorBadRequest("user does not exist"))?;
        }
        let mut user = user.unwrap();

        let mut user: user::ActiveModel = user.into();
        user.name = Set(name);

        let user = user.update(&self.db).await;
        if user.is_err() {
            Err(error::ErrorInternalServerError("cannot update user"))?;
        }
        let user = user.unwrap();

        let serialized = serde_json::to_string(&user);
        if serialized.is_err() {
            Err(error::ErrorInternalServerError("cannot serialize user"))?;
        }
        Ok(serialized.unwrap())
    }

    async fn process_role_update(&self, json: &Value) -> Result<String, Error> {
        let slug = Self::unwrap_json_string(json, "slug")?;
        let name = json.get("name");
        let permissions = json.get("permissions");

        let role = role::Entity::find()
            .filter(role::Column::Slug.eq(&slug))
            .one(&self.db)
            .await;

        if role.is_err() {
            Err(error::ErrorBadRequest("role does not exist"))?;
        }
        let role = role.unwrap();
        if role.is_none() {
            Err(error::ErrorBadRequest("role does not exist"))?;
        }
        let mut role = role.unwrap();

        let mut role: role::ActiveModel = role.into();

        if name.is_some() {
            let name = name.unwrap();
            if !name.is_string() {
                return Err(error::ErrorBadRequest("name is not a string"));
            }
            let name = name.as_str().unwrap();
            role.name = Set(name.to_owned());
        }

        if permissions.is_some() {
            let permissions = permissions.unwrap();
            if !permissions.is_string() {
                return Err(error::ErrorBadRequest("permissions is not a string"));
            }
            let permissions = permissions.as_str().unwrap();
            role.permissions = Set(permissions.to_owned());
        }

        let role = role.update(&self.db).await;
        if role.is_err() {
            Err(error::ErrorInternalServerError("cannot update role"))?;
        }
        let role = role.unwrap();

        let serialized = serde_json::to_string(&role);
        if serialized.is_err() {
            Err(error::ErrorInternalServerError("cannot serialize role"))?;
        }
        Ok(serialized.unwrap())
    }

    async fn process_assign_role(&self, json: &Value) -> Result<String, Error> {
        let slug = Self::unwrap_json_string(json, "slug")?;
        let id = Self::unwrap_json_i32(json, "id")?;

        let role = role::Entity::find()
            .filter(role::Column::Slug.eq(&slug))
            .one(&self.db)
            .await;

        if role.is_err() {
            Err(error::ErrorBadRequest("role does not exist"))?;
        }
        let role = role.unwrap();
        if role.is_none() {
            Err(error::ErrorBadRequest("role does not exist"))?;
        }

        let user_role = users_roles::Entity::find()
            .filter(users_roles::Column::UserId.eq(id))
            .filter(users_roles::Column::RoleSlug.eq(&slug))
            .one(&self.db)
            .await;

        if user_role.is_ok() {
            let user_role = user_role.unwrap();
            if user_role.is_some() {
                Err(error::ErrorBadRequest("Role is already assigned"))
            } else {
                let user_role = users_roles::ActiveModel {
                    user_id: Set(id),
                    role_slug: Set(slug.clone()),
                    ..Default::default()
                };

                let res = user_role.insert(&self.db).await;
                if res.is_err() {
                    return Err(error::ErrorInternalServerError("Database internal error"));
                };

                let serialized = serde_json::to_string(&slug);
                if serialized.is_err() {
                    Err(error::ErrorInternalServerError("cannot serialize role"))?;
                }

                Ok(serialized.unwrap())
            }
        } else {
            Err(error::ErrorInternalServerError("Database internal error"))
        }
    }

    async fn process_unassing_role(&self, json: &Value) -> Result<String, Error> {
        let slug = Self::unwrap_json_string(json, "slug")?;
        let id = Self::unwrap_json_i32(json, "id")?;

        let user_role = users_roles::Entity::find()
            .filter(users_roles::Column::UserId.eq(id))
            .filter(users_roles::Column::RoleSlug.eq(&slug))
            .one(&self.db)
            .await;

        if user_role.is_err() {
            Err(error::ErrorBadRequest("Role is not assigned"))
        } else {
            let user_role = user_role.unwrap();
            if user_role.is_none() {
                Err(error::ErrorBadRequest("Role is not assigned"))
            } else {
                let user_role = user_role.unwrap();
                let user_role = user_role.delete(&self.db).await;
                if user_role.is_err() {
                    Err(error::ErrorInternalServerError("Database internal error"))
                } else {
                    let serialized = serde_json::to_string(&slug);
                    if serialized.is_err() {
                        Err(error::ErrorInternalServerError("cannot serialize role"))?;
                    }
                    Ok(serialized.unwrap())
                }
            }
        }
    }

    async fn process_show_users(&self) -> Result<String, Error> {
        let user_select = user::Entity::find()
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
            .await;

        if user_select.is_err() {
            return Err(error::ErrorInternalServerError("cannot select users"));
        };
        let user_select = user_select.unwrap();

        let serialized = serde_json::to_string(&user_select);
        if serialized.is_err() {
            return Err(error::ErrorInternalServerError("cannot serialize users"));
        }
        Ok(serialized.unwrap())
    }

    async fn process_show_roles(&self) -> Result<String, Error> {
        let roles = entities::role::Entity::find()
            .all(&self.db)
            .await;

        if roles.is_err() {
            return Err(error::ErrorInternalServerError("cannot select roles"));
        }
        let roles = roles.unwrap();

        let serialized = serde_json::to_string(&roles);
        if serialized.is_err() {
            return Err(error::ErrorInternalServerError("cannot serialize roles"));
        }
        Ok(serialized.unwrap())
    }

    async fn process_show_user(&self, json: &Value) -> Result<String, Error> {
        let id = Self::unwrap_json_i32(json, "id")?;

        let user_select = user::Entity::find_by_id(id)
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
            .await;

        if user_select.is_err() {
            return Err(error::ErrorInternalServerError("cannot select users"));
        };
        let user_select = user_select.unwrap();

        let serialized = serde_json::to_string(&user_select);
        if serialized.is_err() {
            return Err(error::ErrorInternalServerError("cannot serialize users"));
        }
        Ok(serialized.unwrap())
    }

    async fn process_show_role(&self, json: &Value) -> Result<String, Error> {
        let slug = Self::unwrap_json_string(json, "slug")?;

        let role = entities::role::Entity::find()
            .filter(role::Column::Slug.eq(slug))
            .one(&self.db)
            .await;

        if role.is_err() {
            return Err(error::ErrorInternalServerError("cannot select role"));
        }
        let role = role.unwrap();
        if role.is_none() {
            return Err(error::ErrorBadRequest("role does not exist"));
        }
        let role = role.unwrap();

        let serialized = serde_json::to_string(&role);
        if serialized.is_err() {
            return Err(error::ErrorInternalServerError("cannot serialize role"));
        }
        Ok(serialized.unwrap())
    }

    async fn process_command(&self, json: &Value, rtype: RequestType) -> Result<String, Error> {
        match rtype {
            RequestType::AddUser => {
                self.process_user_add(json).await
            }
            RequestType::AddRole => {
                self.process_role_add(json).await
            }
            RequestType::DeleteUser => {
                self.process_delete_user(json).await
            }
            RequestType::DeleteRole => {
                self.process_role_delete(json).await
            }
            RequestType::UpdateUser => {
                self.process_user_update(json).await
            }
            RequestType::UpdateRole => {
                self.process_role_update(json).await
            }
            RequestType::AssignRole => {
                self.process_assign_role(json).await
            }
            RequestType::UnassignRole => {
                self.process_unassing_role(json).await
            }
            RequestType::ShowUsers => {
                self.process_show_users().await
            }
            RequestType::ShowRoles => {
                self.process_show_roles().await
            }
            RequestType::ShowUser => {
                self.process_show_user(json).await
            }
            RequestType::ShowRole => {
                self.process_show_role(json).await
            }
        }
    }

    pub async fn process_request(&self, json: Value) -> Result<String, Error> {
        let request_type = Self::get_command_type(&json)?;
        let res = self.process_command(&json, request_type).await?;

        Ok(res)
    }

}