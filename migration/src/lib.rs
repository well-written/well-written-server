pub use sea_orm_migration::prelude::*;

mod m20221001_204938_create_user_table;
mod m20221002_055451_create_blog_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221001_204938_create_user_table::Migration),
            Box::new(m20221002_055451_create_blog_table::Migration),
        ]
    }
}
