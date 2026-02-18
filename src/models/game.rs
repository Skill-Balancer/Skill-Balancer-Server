use ormlite::model::*;

#[derive(Model, Debug)]
#[ormlite(table = "Game")]
pub struct Game {
    #[ormlite(primary_key)]
    pub player_one_id: u32,
    pub player_two_id: u32,
}