use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Blog::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_owned()),
                    )
                    .col(ColumnDef::new(Blog::Title).text().not_null())
                    .col(ColumnDef::new(Blog::Description).text().not_null())
                    .col(
                        ColumnDef::new(Blog::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT now()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Blog::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Role::Table)
                    .values([Role::Owner, Role::Admin, Role::Writer])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BlogMembership::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogMembership::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_owned()),
                    )
                    .col(ColumnDef::new(BlogMembership::UserId).uuid().not_null())
                    .col(ColumnDef::new(BlogMembership::BlogId).uuid().not_null())
                    .col(ColumnDef::new(BlogMembership::Role).enumeration(
                        Role::Table.to_string(),
                        [
                            Role::Owner.to_string(),
                            Role::Admin.to_string(),
                            Role::Writer.to_string(),
                        ],
                    ))
                    .col(
                        ColumnDef::new(BlogMembership::JoinedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT now()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(BlogMembership::LeftAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_membership_user_id_fkey")
                            .from(BlogMembership::Table, BlogMembership::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_membership_blog_id_fkey")
                            .from(BlogMembership::Table, BlogMembership::BlogId)
                            .to(Blog::Table, Blog::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BlogMembership::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Role::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Blog::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Blog {
    Table,
    Id,
    Title,
    Description,
    CreatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum BlogMembership {
    Table,
    Id,
    UserId,
    BlogId,
    Role,
    JoinedAt,
    LeftAt,
}

#[derive(Iden)]
enum Role {
    Table,
    Owner,
    Admin,
    Writer,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
