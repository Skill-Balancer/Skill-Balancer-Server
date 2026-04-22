use std::time::Duration;

use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait};

use crate::{
    entities::config,
    env::{self, data_dir},
    storage::utils::create_dir,
};

#[derive(Clone)]
pub struct DB {
    connection: DatabaseConnection,
}

impl DB {
    pub async fn new() -> Result<Self, DbErr> {
        create_dir(&data_dir());
        let mut opt = ConnectOptions::new(format!("{}?mode=rwc", env::db_url()));
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8));

        Ok(Self {
            connection: Database::connect(opt).await?,
        })
    }
    #[allow(unused)]
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn sync_schema(&self) -> Result<(), DbErr> {
        self.connection
            .get_schema_registry("Skill-Balancer-Server::entities::*")
            .sync(&self.connection)
            .await
    }

    pub async fn close(self) -> Result<(), DbErr> {
        self.connection.close().await
    }

    pub async fn insert_config(&self, config: config::ActiveModel) -> Result<config::Model, DbErr> {
        config.insert(&self.connection).await
    }

    pub async fn update_config(&self, config: config::ActiveModel) -> Result<config::Model, DbErr> {
        config.update(&self.connection).await
    }

    #[allow(unused)]
    pub async fn get_config(&self, id: &String) -> Result<Option<config::Model>, DbErr> {
        config::Entity::find_by_id(id).one(&self.connection).await
    }

    pub async fn get_all_configs(&self) -> Result<Vec<config::Model>, DbErr> {
        config::Entity::find().all(&self.connection).await
    }
}
