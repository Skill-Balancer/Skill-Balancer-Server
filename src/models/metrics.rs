use colored::*;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub step: usize,
    pub mean_reward: f32,
    pub policy_loss: f32,
    pub value_loss: f32,
    pub entropy: f32,
    pub clip_fraction: f32,
}

impl Metrics {
    pub fn print_pretty(&self) {
        println!(
            "{}",
            format!("─── Step {} ───", self.step).bright_cyan().bold()
        );

        println!(
            "  {} {:<14} {:>10.6}",
            "📈".green(),
            "Mean Reward:",
            self.mean_reward.to_string().green()
        );

        println!(
            "  {} {:<14} {:>10.6}",
            "📉".red(),
            "Policy Loss:",
            self.policy_loss.to_string().red()
        );
        println!(
            "  {} {:<14} {:>10.6}",
            "📉".red(),
            "Value Loss:",
            self.value_loss.to_string().red()
        );

        println!(
            "  {} {:<14} {:>10.6}",
            "🧠".blue(),
            "Entropy:",
            self.entropy.to_string().blue()
        );

        let clip_color = if self.clip_fraction > 0.2 {
            "yellow"
        } else {
            "white"
        };
        println!(
            "  {} {:<14} {:>10.6}",
            "📎".white(),
            "Clip Fraction:",
            self.clip_fraction.to_string().color(clip_color)
        );

        println!("{}", "─".repeat(25).bright_black());
    }
}
