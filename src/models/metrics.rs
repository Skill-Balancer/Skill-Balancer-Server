#[derive(Debug, Clone)]
pub struct Metrics {
    pub step: usize,
    pub mean_reward: f32,
    pub policy_loss: f32,
    pub value_loss: f32,
    pub entropy: f32,
    pub clip_fraction: f32,
}
