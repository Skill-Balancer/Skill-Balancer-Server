use burn_rl::base::ElemType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub name: String,
    pub description: Option<String>,
    pub state_size: u32,
    pub action_size: u32,
    pub train_every: u32,
    pub gamma: ElemType,
    pub lambda: ElemType,
    pub epsilon_clip: ElemType,
    pub critic_weight: ElemType,
    pub entropy_weight: ElemType,
    pub learning_rate: ElemType,
    pub epochs: u32,
    pub batch_size: u32,
    pub clip_grad: f32,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
