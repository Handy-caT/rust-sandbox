use clap::ArgMatches;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use entities::{role, user, users_roles};
use crate::requests::{AddRoleRequest, AddUserRequest, AssignRoleRequest, DeleteRoleRequest, DeleteUserRequest, RequestType, UnassignRoleRequest, UpdateRoleRequest, UpdateUserRequest};

pub struct CliProcessor {
    matches: ArgMatches
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(pub i32);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleSlug(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserName(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleName(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolePermissions(pub String);



pub enum CliCommand {
    AddUser(AddUserRequest),
    AddRole(AddRoleRequest),
    DeleteUserId(DeleteUserRequest),
    DeleteRoleSlug(DeleteRoleRequest),
    UpdateUser(UpdateUserRequest),
    UpdateRole(UpdateRoleRequest),
    AssignRole(AssignRoleRequest),
    UnassignRole(UnassignRoleRequest),
    ShowUsers,
    ShowRoles,
    ShowUser(UserId),
    ShowRole(RoleSlug),
}

impl CliProcessor {
    pub fn new(matches: ArgMatches) -> Self {
        Self {
            matches
        }
    }

    fn process_add(matches: &ArgMatches) -> CliCommand {
        match matches.subcommand() {
            Some(("user", sub_matches)) => {
                let name = sub_matches.get_one::<String>("name");
                let role_slug = sub_matches.get_one::<String>("role");

                let request = AddUserRequest {
                    command_type: RequestType::AddUser,
                    name: UserName(name.unwrap().to_owned()),
                    role: RoleSlug(role_slug.unwrap().to_owned()),
                };

                CliCommand::AddUser(request)
            }
            Some(("role", sub_matches)) => {
                let slug = sub_matches.get_one::<String>("slug");
                let name = sub_matches.get_one::<String>("name");
                let permissions = sub_matches.get_one::<String>("permissions");

                let request = AddRoleRequest {
                    command_type: RequestType::AddRole,
                    slug: RoleSlug(slug.unwrap().to_owned()),
                    name: name.map(|name| RoleName(name.to_owned())),
                    permissions: permissions.map(|permissions| RolePermissions(permissions.to_owned())),
                };

                CliCommand::AddRole(request)
            }
            _ => unreachable!()
        }
    }

    fn process_delete(matches: &ArgMatches) -> CliCommand {
        match matches.subcommand() {
            Some(("user", sub_matches)) => {
                let id = sub_matches.get_one::<i32>("id");

                let request = DeleteUserRequest {
                    command_type: RequestType::DeleteUser,
                    id: id.unwrap().to_owned(),
                };

                CliCommand::DeleteUserId(request)
            }
            Some(("role", sub_matches)) => {
                let slug = sub_matches.get_one::<String>("slug");

                let request = DeleteRoleRequest {
                    command_type: RequestType::DeleteRole,
                    slug: slug.unwrap().to_owned(),
                };

                CliCommand::DeleteRoleSlug(request)
            }
            _ => unreachable!()
        }
    }

    fn process_update(matches: &ArgMatches) -> CliCommand {
        match matches.subcommand() {
            Some(("user", sub_matches)) => {
                let id = sub_matches.get_one::<i32>("id");
                let name = sub_matches.get_one::<String>("name");

                let request = UpdateUserRequest {
                    command_type: RequestType::UpdateUser,
                    id: id.unwrap().to_owned(),
                    name: UserName(name.unwrap().to_owned()),
                };
                
                CliCommand::UpdateUser(request)
            }
            Some(("role", sub_matches)) => {
                let slug = sub_matches.get_one::<String>("slug");
                let name = sub_matches.get_one::<String>("name");
                let permissions = sub_matches.get_one::<String>("permissions");

                let request = UpdateRoleRequest {
                    command_type: RequestType::UpdateRole,
                    slug: RoleSlug(slug.unwrap().to_owned()),
                    name: name.map(|name| RoleName(name.to_owned())),
                    permissions: permissions.map(|permissions| RolePermissions(permissions.to_owned())),
                };
                
                CliCommand::UpdateRole(request)
            }
            _ => unreachable!()
        }
    }

    fn process_assign(matches: &ArgMatches) -> CliCommand {
        let id = matches.get_one::<i32>("user_id");
        let slug = matches.get_one::<String>("role_slug");

        let request = AssignRoleRequest {
            command_type: RequestType::AssignRole,
            id: id.unwrap().to_owned(),
            slug: RoleSlug(slug.unwrap().to_owned())
        };

        CliCommand::AssignRole(request)
    }

    fn process_unassign(matches: &ArgMatches) -> CliCommand {
        let id = matches.get_one::<i32>("user_id");
        let slug = matches.get_one::<String>("role_slug");

        let request = UnassignRoleRequest {
            command_type: RequestType::UnassignRole,
            id: id.unwrap().to_owned(),
            slug: RoleSlug(slug.unwrap().to_owned())
        };

        CliCommand::UnassignRole(request)
    }

    fn process_show(matches: &ArgMatches) -> CliCommand {
        match matches.subcommand() {
            Some(("user", sub_matches)) => {
                let id = sub_matches.get_one::<i32>("id");
                if id.is_none() {
                    return CliCommand::ShowUsers;
                }

                CliCommand::ShowUser(UserId(id.unwrap().to_owned()))
            }
            Some(("role", sub_matches)) => {
                let slug = sub_matches.get_one::<String>("slug");
                if slug.is_none() {
                    return CliCommand::ShowRoles;
                }

                CliCommand::ShowRole(RoleSlug(slug.unwrap().to_owned()))
            }
            _ => unreachable!()
        }
    }

    pub fn process_cli(&self) -> CliCommand {
        match self.matches.subcommand() {
            Some(("add",  sub_matches)) => {
                CliProcessor::process_add(sub_matches)
            }
            Some(("delete", sub_matches)) => {
                CliProcessor::process_delete(sub_matches)
            }
            Some(("update", sub_matches)) => {
                CliProcessor::process_update(sub_matches)
            }
            Some(("assign", sub_matches)) => {
                CliProcessor::process_assign(sub_matches)
            }
            Some(("unassign", sub_matches)) => {
                CliProcessor::process_unassign(sub_matches)
            }
            Some(("show", sub_matches)) => {
                CliProcessor::process_show(sub_matches)
            }
            _ => unreachable!()
        }
    }
}