use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .unique_key()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name)
                        .string()
                        .not_null()
                    )
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Role::Slug)
                        .string()
                        .not_null()
                        .unique_key()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Role::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(Role::Permissions)
                        .string()
                        .not_null()
                    )
                    .to_owned(),
            )
            .await?;

        manager.
            create_table(
                Table::create()
                    .table(UsersRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UsersRoles::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(UsersRoles::UserId)
                        .integer()
                        .not_null()
                    )
                    .col(ColumnDef::new(UsersRoles::RoleSlug)
                        .string()
                        .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_roles_user_id")
                            .to_tbl(User::Table)
                            .to_col(User::Id)
                            .from_tbl(UsersRoles::Table)
                            .from_col(UsersRoles::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_roles_role_slug")
                            .to_tbl(Role::Table)
                            .to_col(Role::Slug)
                            .from_tbl(UsersRoles::Table)
                            .from_col(UsersRoles::RoleSlug)
                            .to_owned()
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;

        manager.drop_table(Table::drop().table(UsersRoles::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    RoleSlug
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Slug,
    Name,
    Permissions,
}

#[derive(DeriveIden)]
pub enum UsersRoles {
    #[sea_orm(iden = "users_roles")]
    Table,
    Id,
    UserId,
    RoleSlug,
}