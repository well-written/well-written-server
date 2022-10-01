use sea_orm::DatabaseConnection;

pub mod entities;
pub mod schema;

pub struct OrmDataloader {
  pub db: DatabaseConnection,
}
