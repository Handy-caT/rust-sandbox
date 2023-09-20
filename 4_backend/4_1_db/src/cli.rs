use clap::{Arg, ArgAction, ArgGroup, Command, value_parser};

pub fn cli() -> Command {
    Command::new("db")
        .about("cli using for db access")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("add new object to the database")
                .subcommand(
                    Command::new("user")
                        .about("add new user to the database")
                        .arg(Arg::new("name")
                            .help("name of the user to add")
                            .required(true)
                            .long("name")
                            .short('n')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(String))
                            .num_args(1)
                        )
                        .arg(Arg::new("role")
                            .help("role slug's to add to the user")
                            .required(true)
                            .long("role")
                            .short('r')
                            .action(ArgAction::Append)
                            .value_parser(value_parser!(String))
                            .num_args(1..)
                        )
                )
                .subcommand(
                    Command::new("role")
                        .about("add new role to the database")
                        .arg(Arg::new("name")
                            .help("name of the role to add")
                            .required(true)
                            .long("name")
                            .short('n')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(String))
                            .num_args(1)
                        )
                        .arg(Arg::new("slug")
                            .help("slug of a role, must be unique between roles")
                            .required(true)
                            .long("slug")
                            .short('s')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(String))
                            .num_args(1)
                        )
                        .arg(Arg::new("permissions")
                            .help("roles permissions")
                            .required(true)
                            .long("decs")
                            .short('d')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(String))
                            .num_args(1)
                        )
                )

        )
        .subcommand(
            Command::new("delete")
                .about("delete object from the database")
                .subcommand(
                    Command::new("user")
                        .about("delete user from the database")
                        .arg(Arg::new("id")
                            .help("id of a user to delete")
                            .required(true)
                            .long("id")
                            .short('i')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(i32))
                            .num_args(1)
                        )
                )
                .subcommand(
                    Command::new("role")
                        .about("delete role from the database")
                        .arg(Arg::new("slug")
                            .help("slug of a role, must be unique between roles")
                            .required(true)
                            .long("slug")
                            .short('s')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(String))
                            .num_args(1)
                        )
                )
        )
        .subcommand(Command::new("update")
            .about("update object in the database")
            .subcommand(
                Command::new("user")
                    .about("update user by id")
                    .arg(Arg::new("id")
                        .help("id of a user to update")
                        .required(true)
                        .long("id")
                        .short('i')
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i32))
                        .num_args(1)
                    )
                    .arg(Arg::new("name")
                        .help("new name of the user")
                        .long("name")
                        .short('n')
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String))
                        .num_args(1)
                    )
            )
            .subcommand(
                Command::new("role")
                    .about("update role by slug")
                    .arg(Arg::new("slug")
                        .help("slug of a role to update")
                        .required(true)
                        .long("slug")
                        .short('s')
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String))
                        .num_args(1)
                    )
                    .arg(Arg::new("name")
                        .help("new name of the role")
                        .long("name")
                        .short('n')
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String))
                        .num_args(1)
                    )
                    .arg(
                        Arg::new("permissions")
                            .help("new permissions of the role")
                            .long("permissions")
                            .short('p')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(String))
                            .num_args(1)
                    )
                    .group(ArgGroup::new("update")
                        .args(["name", "permissions"])
                        .required(true)
                        .multiple(true)
                    )
            )
        )
        .subcommand(
            Command::new("assign")
                .about("assign role to user")
                .arg(Arg::new("user_id")
                    .help("id of a user to assign role")
                    .required(true)
                    .long("user-id")
                    .short('u')
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(i32))
                    .num_args(1)
                )
                .arg(Arg::new("role_slug")
                    .help("slug of a role to assign")
                    .required(true)
                    .long("role-slug")
                    .short('r')
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(String))
                    .num_args(1)
                )
        )
        .subcommand(
            Command::new("unassign")
                .about("unassign role from user")
                .arg(Arg::new("user_id")
                    .help("id of a user to unassign role")
                    .required(true)
                    .long("user-id")
                    .short('u')
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(i32))
                    .num_args(1)
                )
                .arg(Arg::new("role_slug")
                    .help("slug of a role to unassign")
                    .required(true)
                    .long("role-slug")
                    .short('r')
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(String))
                    .num_args(1)
                )
        )
        .subcommand(
            Command::new("show")
                .about("show object or a list of objects from the database")
                .subcommand(
                    Command::new("user")
                        .about("show user by id if present or all users")
                        .arg(Arg::new("id")
                            .help("id of a user to show")
                            .required(false)
                            .long("id")
                            .short('i')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(i32))
                            .num_args(1)
                        )
                )
                .subcommand(
                    Command::new("role")
                        .about("show role by slug if present or all roles")
                        .arg(Arg::new("slug")
                            .help("slug of a role to show")
                            .required(false)
                            .long("slug")
                            .short('s')
                            .action(ArgAction::Set)
                            .value_parser(value_parser!(String))
                            .num_args(1)
                        )
                )
        )
}