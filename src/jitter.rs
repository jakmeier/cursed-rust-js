use crate::MyBotDetection;

impl MyBotDetection {
    /// Returns a number between 0.0 and 1.0, with higher number meaning there
    /// was more jitter.
    pub(crate) fn jitter(&self) -> f32 {
        // just for demonstration, compute the variance of 2D differences
        // between two events

        // I was too lazy to code this myself, it's all ChatGPT from here:
        // <ChatGPT>
        if self.events.len() < 2 {
            return 0.0; // Not enough events to compute differences
        }

        let mut diffs = Vec::new();

        // Calculate 2D differences between consecutive events
        for i in 1..self.events.len() {
            let dx = (self.events[i].coordinate.x - self.events[i - 1].coordinate.x) as f32;
            let dy = (self.events[i].coordinate.y - self.events[i - 1].coordinate.y) as f32;
            let dist = (dx * dx + dy * dy).sqrt(); // Euclidean distance
            diffs.push(dist);
        }

        // Compute mean of differences
        let mean: f32 = diffs.iter().sum::<f32>() / diffs.len() as f32;

        // Compute variance
        let variance: f32 =
            diffs.iter().map(|&d| (d - mean).powi(2)).sum::<f32>() / diffs.len() as f32;

        // <ChatGPT/>

        // normalize
        (variance / 1000.0).min(1.0)
    }
}
