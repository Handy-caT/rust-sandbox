use sea_orm_migration::prelude::*;
use sea_orm_migration::seaql_migrations::PrimaryKey;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
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
                        .unique_key()
                    )
                    .col(ColumnDef::new(User::PasswordHash)
                        .string()
                    )
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(UserFriend::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserFriend::UserId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserFriend::FriendId)
                            .integer()
                            .not_null()
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_users_friends")
                            .col(UserFriend::UserId)
                            .col(UserFriend::FriendId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_friends_user_id")
                            .to_tbl(User::Table)
                            .to_col(User::Id)
                            .from_tbl(UserFriend::Table)
                            .from_col(UserFriend::FriendId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_friends_friend_id")
                            .to_tbl(User::Table)
                            .to_col(User::Id)
                            .from_tbl(UserFriend::Table)
                            .from_col(UserFriend::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(
                Table::drop()
                    .table(UserFriend::Table)
                    .if_exists()
                    .to_owned(),
            ).await?;

        manager
            .drop_table(
                Table::drop()
                    .table(User::Table)
                    .if_exists()
                    .to_owned(),
            ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    PasswordHash,
}

#[derive(DeriveIden)]
enum UserFriend {
    #[sea_orm(iden = "users_friends")]
    Table,
    UserId,
    FriendId,
}
