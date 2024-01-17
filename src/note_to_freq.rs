use std::collections::HashMap;

pub fn note_to_freq(note_name: String) -> Option<f32> {
    let note_map: HashMap<String, i32> = [
        ("C".to_string(), 0),
        ("C#".to_string(), 1),
        ("D".to_string(), 2),
        ("D#".to_string(), 3),
        ("E".to_string(), 4),
        ("F".to_string(), 5),
        ("F#".to_string(), 6),
        ("G".to_string(), 7),
        ("G#".to_string(), 8),
        ("A".to_string(), 9),
        ("A#".to_string(), 10),
        ("B".to_string(), 11),
    ]
    .iter()
    .cloned()
    .collect();

    let mut chars = note_name.chars();
    let note = chars.next()?.to_string();
    let octave: i32 = chars.collect::<String>().parse().ok()?;
    let scale_degree = note_map.get(&note)?;

    Some(2.0f32.powf((*scale_degree as f32 + (octave - 4) as f32 * 12.0) / 12.0) * 440.0)
}