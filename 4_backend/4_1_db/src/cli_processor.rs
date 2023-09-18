use clap::ArgMatches;
use sea_orm::ActiveValue::Set;
use entities::{role, user, users_roles};

pub struct CliProcessor {
    matches: ArgMatches
}

pub struct UserId(pub i32);
pub struct RoleSlug(pub String);
pub struct UserName(pub String);
pub struct RoleName(pub String);
pub struct RolePermissions(pub String);


pub enum CliCommand {
    AddUser(user::ActiveModel, users_roles::ActiveModel),
    AddRole(role::ActiveModel),
    DeleteUserId(UserId),
    DeleteRoleSlug(RoleSlug),
    UpdateUser(UserId, UserName),
    UpdateRole(RoleSlug, Option<RoleName>, Option<RolePermissions>),
    AssignRole(UserId, RoleSlug),
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

                let user = user::ActiveModel {
                    name: Set(name.unwrap().to_owned()),
                    ..Default::default()
                };

                let user_role = users_roles::ActiveModel {
                    role_slug: Set(role_slug.unwrap().to_owned()),
                    ..Default::default()
                };

                CliCommand::AddUser(user, user_role)
            }
            Some(("role", sub_matches)) => {
                let slug = sub_matches.get_one::<String>("slug");
                let name = sub_matches.get_one::<String>("name");
                let permissions = sub_matches.get_one::<String>("permissions");

                let role = role::ActiveModel {
                    slug: Set(slug.unwrap().to_owned()),
                    name: Set(name.unwrap().to_owned()),
                    permissions: Set(permissions.unwrap().to_owned()),
                    ..Default::default()
                };

                CliCommand::AddRole(role)
            }
            _ => unreachable!()
        }
    }

    fn process_delete(matches: &ArgMatches) -> CliCommand {
        match matches.subcommand() {
            Some(("user", sub_matches)) => {
                let id = sub_matches.get_one::<i32>("id");

                CliCommand::DeleteUserId(UserId(id.unwrap().to_owned()))
            }
            Some(("role", sub_matches)) => {
                let slug = sub_matches.get_one::<String>("slug");

                CliCommand::DeleteRoleSlug(RoleSlug(slug.unwrap().to_owned()))
            }
            _ => unreachable!()
        }
    }

    fn process_update(matches: &ArgMatches) -> CliCommand {
        match matches.subcommand() {
            Some(("user", sub_matches)) => {
                let id = sub_matches.get_one::<i32>("id");
                let name = sub_matches.get_one::<String>("name");

                CliCommand::UpdateUser(UserId(id.unwrap().to_owned()), UserName(name.unwrap().to_owned()))
            }
            Some(("role", sub_matches)) => {
                let slug = sub_matches.get_one::<String>("slug");
                let name = sub_matches.get_one::<String>("name");
                let permissions = sub_matches.get_one::<String>("permissions");

                CliCommand::UpdateRole(RoleSlug(slug.unwrap().to_owned()),
                                       name.map(|name| RoleName(name.to_owned())),
                                       permissions.map(|permissions| RolePermissions(permissions.to_owned())))
            }
            _ => unreachable!()
        }
    }

    fn process_assign(matches: &ArgMatches) -> CliCommand {
        let id = matches.get_one::<i32>("user_id");
        let slug = matches.get_one::<String>("role_slug");

        CliCommand::AssignRole(UserId(id.unwrap().to_owned()), RoleSlug(slug.unwrap().to_owned()))
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
            Some(("show", sub_matches)) => {
                CliProcessor::process_show(sub_matches)
            }
            _ => unreachable!()
        }
    }
}