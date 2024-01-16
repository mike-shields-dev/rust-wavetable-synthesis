use std::time::Duration;
use rodio::{ OutputStream, Source };
use clap::Parser;
use waveforms::{render_pulse, render_saw, render_sin, render_triangle};

mod waveforms;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]

struct Args {
    #[arg(short, long, default_value_t = ("sine").to_string())]
    waveform: String,
    #[arg(short, long, default_value_t = 0.5)]
    gain: f32,
    #[arg(short, long, default_value_t = 1.0)]
    duration: f32,
}

struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    gain: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
            gain: 1.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn set_gain(&mut self, gain: f32) {
        if gain >= -1.0 && gain <= 1.0 {
            self.gain = gain;
        }
    }

    fn get_sample(&mut self) -> f32 {
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

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

fn main() {
    let args = Args::parse();
    let waveform_type = args.waveform.as_str();
    let gain = args.gain;
    let duration: f32 = args.duration;

    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    match waveform_type {
        "sine" => render_sin(&mut wave_table, wave_table_size),
        "saw" => render_saw(&mut wave_table, wave_table_size),
        "triangle" => render_triangle(&mut wave_table, wave_table_size),
        "pulse" => render_pulse(&mut wave_table, wave_table_size),
        _ => render_sin(&mut wave_table, wave_table_size)
    }   

    let mut oscillator = WavetableOscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0);
    oscillator.set_gain(gain);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_millis((1000.0 * duration.abs()) as u64));
}
