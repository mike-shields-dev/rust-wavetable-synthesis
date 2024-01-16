pub fn render_sin(wave_table: &mut Vec<f32>, wave_table_size: usize) {
    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }
}

pub fn render_saw(wave_table: &mut Vec<f32>, wave_table_size: usize) {
    for n in 0..wave_table_size {
        let sawtooth_value = 2.0 * (n as f32 / wave_table_size as f32) - 1.0;
        wave_table.push(sawtooth_value);
    }
}

pub fn render_triangle(wave_table: &mut Vec<f32>, wave_table_size: usize) {
    for n in 0..wave_table_size {
        let triangle_value = 2.0 * (2.0 * (n as f32 / wave_table_size as f32) - 1.0).abs() - 1.0;
        wave_table.push(triangle_value);
    }
}

pub fn render_pulse(wave_table: &mut Vec<f32>, wave_table_size: usize) {
    let duty_cycle = 0.5;
    for n in 0..wave_table_size {
        let square_value = if (n as f32 / wave_table_size as f32) < duty_cycle {
            1.0
        } else {
            -1.0
        };
        wave_table.push(square_value);
    }
}