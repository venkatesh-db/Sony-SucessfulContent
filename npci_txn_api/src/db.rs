use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn init() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&url).await
}
