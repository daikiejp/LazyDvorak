use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Stats {
    pub correct: u32,
    pub errors: u32,
    pub total_chars: u32,
    pub start_time: Option<Instant>,
    pub completed_exercises: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            correct: 0,
            errors: 0,
            total_chars: 0,
            start_time: None,
            completed_exercises: 0,
        }
    }

    pub fn reset(&mut self) {
        self.correct = 0;
        self.errors = 0;
        self.total_chars = 0;
        self.start_time = None;
        self.completed_exercises = 0;
    }

    // TODO!: Calculate WPM
    // WPM = (correct_chars / 5) / (time_in_minutes)
    // This assumes an average word length of 5 characters
    pub fn calculate_wpm(&self) -> f32 {
        if let Some(start) = self.start_time {
            let elapsed_minutes = start.elapsed().as_secs_f32() / 60.0;
            if elapsed_minutes > 0.0 {
                let words = (self.correct as f32) / 5.0;
                return words / elapsed_minutes;
            }
        }
        0.0
    }

    pub fn calculate_accuracy(&self) -> f32 {
        let total_attempts = self.correct + self.errors;
        if total_attempts > 0 {
            (self.correct as f32 / total_attempts as f32) * 100.0
        } else {
            100.0
        }
    }
}
