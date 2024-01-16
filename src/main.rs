use clap::Parser;
use rodio::{OutputStream, Source};
use std::time::Duration;
mod waveforms;
use waveforms::render_waveform;
mod wavetable_oscillator;
use wavetable_oscillator::WavetableOscillator;

#[derive(Parser, Debug)]

struct Args {
    #[arg(short, long, default_value_t = String::from("sine"))]
    waveform: String,
    #[arg(short, long, default_value_t = 0.5)]
    gain: f64,
    #[arg(short, long, default_value_t = 1000)]
    milliseconds: u64,
    #[arg(short, long, default_value_t = 50.0)]
    duty: f64,
}

fn main() {
    let args = Args::parse();
    let waveform_type = args.waveform.as_str();
    let duration_millis = args.milliseconds;
    let duty = args.duty;
    let wave_table_size = 64;
    let gain_limit = 1.0;
    let gain = args.gain.max(-gain_limit).min(gain_limit);
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    render_waveform(waveform_type, &mut wave_table, wave_table_size, duty);

    let mut oscillator = WavetableOscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0);
    oscillator.set_gain(gain);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_millis(duration_millis.abs_diff(0)));
}
