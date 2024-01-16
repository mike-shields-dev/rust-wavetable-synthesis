use rodio::Source;
use std::time::Duration;

pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    gain: f64,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
            gain: 1.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    pub fn set_gain(&mut self, gain: f64) {
        if gain >= -1.0 && gain <= 1.0 {
            self.gain = gain;
        }
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.linear_interpolate() * self.gain as f32;
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn linear_interpolate(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.get_sample())
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
