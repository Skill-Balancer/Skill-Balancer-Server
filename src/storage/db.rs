use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::env::db_url;

pub fn create_database() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(db_url());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    Database::connect(opt)
}

pub fn close_database(db: DatabaseConnection) {
    db.close();
}
