use burn_rl::base::ElemType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "config")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    name: String,
    description: Option<String>,
    gamma: ElemType,
    lambda: ElemType,
    epsilon_clip: ElemType,
    critic_weight: ElemType,
    entropy_weight: ElemType,
    learning_rate: ElemType,
    epochs: u32,
    batch_size: u32,
    clip_grad: u32,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
