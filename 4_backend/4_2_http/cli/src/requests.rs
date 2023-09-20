use serde::{Deserialize, Serialize};
use crate::cli_processor::{RoleName, RolePermissions, RoleSlug, UserId, UserName};

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddUserRequest {
    pub command_type: RequestType,
    pub name: UserName,
    pub role: RoleSlug,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddRoleRequest {
    pub command_type: RequestType,
    pub slug: RoleSlug,
    pub name: Option<RoleName>,
    pub permissions: Option<RolePermissions>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub command_type: RequestType,
    pub id: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteRoleRequest {
    pub command_type: RequestType,
    pub slug: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub command_type: RequestType,
    pub id: i32,
    pub name: UserName,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    pub command_type: RequestType,
    pub slug: RoleSlug,
    pub name: Option<RoleName>,
    pub permissions: Option<RolePermissions>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub command_type: RequestType,
    pub id: i32,
    pub slug: RoleSlug,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnassignRoleRequest {
    pub command_type: RequestType,
    pub id: i32,
    pub slug: RoleSlug,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShowUserRequest {
    pub command_type: RequestType,
    pub id: UserId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShowRoleRequest {
    pub command_type: RequestType,
    pub slug: RoleSlug,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShowUsersRequest {
    pub command_type: RequestType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShowRolesRequest {
    pub command_type: RequestType,
}