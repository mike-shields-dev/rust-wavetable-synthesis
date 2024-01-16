pub fn render_waveform(waveform_type: &str, wave_table: &mut Vec<f32>, wave_table_size: usize, _duty: f64) {
    match waveform_type {
        "sine" => render_sine(wave_table, wave_table_size),
        "saw" => render_saw(wave_table, wave_table_size),
        "triangle" => render_triangle(wave_table, wave_table_size),
        "pulse" => render_pulse(wave_table, wave_table_size, _duty),
        _ => render_sine(wave_table, wave_table_size),
    }
}

pub fn render_sine(wave_table: &mut Vec<f32>, wave_table_size: usize) {
    for n in 0..wave_table_size {
        let sample = (2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin();
        wave_table.push(sample);
    }
}

pub fn render_saw(wave_table: &mut Vec<f32>, wave_table_size: usize) {
    for n in 0..wave_table_size {
        let sample = 2.0 * (n as f32 / wave_table_size as f32) - 1.0;
        wave_table.push(sample);
    }
}

pub fn render_triangle(wave_table: &mut Vec<f32>, wave_table_size: usize) {
    for n in 0..wave_table_size {
        let sample = 2.0 * (2.0 * (n as f32 / wave_table_size as f32) - 1.0).abs() - 1.0;
        wave_table.push(sample);
    }
}

pub fn render_pulse(wave_table: &mut Vec<f32>, wave_table_size: usize, duty: f64) {
    for n in 0..wave_table_size {
        let sample: f32 = if (n as f64 / wave_table_size as f64) * 100.0 < duty {
            1.0
        } else {
            -1.0
        };
        wave_table.push(sample);
    }
}
