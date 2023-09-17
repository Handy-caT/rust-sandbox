use clap::ArgMatches;
use sea_orm::ActiveValue::Set;
use entities::{role, user, users_roles};

pub struct CliProcessor {
    matches: ArgMatches
}

pub enum CliCommand {
    AddUser(user::ActiveModel, users_roles::ActiveModel),
    AddRole(role::ActiveModel),
    DeleteUserName(String),
    DeleteUserId(i32),
    DeleteRoleSlug(String),
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
                let name = sub_matches.get_one::<String>("name");
                let id = sub_matches.get_one::<i32>("id");

                if name.is_some() {
                    CliCommand::DeleteUserName(name.unwrap().to_owned())
                } else if id.is_some() {
                    CliCommand::DeleteUserId(id.unwrap().to_owned())
                } else {
                    unreachable!()
                }
            }
            Some(("role", sub_matches)) => {
                let slug = sub_matches.get_one::<String>("slug");

                CliCommand::DeleteRoleSlug(slug.unwrap().to_owned())
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
            _ => unreachable!()
        }
    }
}