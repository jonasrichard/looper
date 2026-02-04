use std::time::Duration;

use eframe::egui;

#[derive(Debug, Default)]
pub struct Track {
    id: u32,
    name: String,
    length: Duration, // TODO content and length
}

#[derive(Debug, Default)]
pub struct Recording {
    tracks: Vec<Track>,
    /// The metronome bpm speed
    bpm: u32,
    /// The current cursor position in the recording
    cursor_position: Duration,
    /// How much pixels represent a single beat
    beat_in_px: u32,
}

impl Track {
    fn new(id: u32, name: &str, length: Duration) -> Self {
        Self {
            id,
            name: name.to_string(),
            length,
        }
    }
}

impl Recording {
    pub fn new(bpm: u32) -> Self {
        Self {
            tracks: Vec::new(),
            bpm,
            cursor_position: Duration::from_secs(0),
            beat_in_px: 100,
        }
    }

    pub fn add_track(&mut self) {
        let new_id = self.tracks.len() as u32 + 1;
        let new_track = Track::new(new_id, &format!("Track {}", new_id), Duration::from_secs(0));
        self.tracks.push(new_track);
    }

    pub fn remove_track(&mut self) {
        self.tracks.pop();
    }

    pub fn step_cursor(&mut self, seconds: f64) {
        let step = Duration::from_secs_f64(seconds.abs());
        if seconds.is_sign_positive() {
            self.cursor_position += step;
        } else {
            self.cursor_position = self.cursor_position.saturating_sub(step);
        }
    }

    pub fn draw_track_grid(&self, ui: &mut eframe::egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let num_beats = (self.cursor_position.as_secs_f64() * (self.bpm as f64 / 60.0)) as u32;

        for beat in 0..=num_beats {
            let x_pos = rect.left() + (beat * self.beat_in_px) as f32;

            ui.painter().vline(
                x_pos as f32,
                rect.y_range(),
                egui::Stroke::new(1.0, egui::Color32::GRAY),
            );
        }

        ui.painter().vline(
            self.cursor_position.as_secs_f64() as f32 * (self.bpm as f32 / 60.0) * self.beat_in_px as f32 + rect.left(),
            rect.y_range(),
            egui::Stroke::new(2.0, egui::Color32::RED),
        );

        for track in &self.tracks {
            ui.vertical(|ui| {
                ui.label(format!("Track {}: {}", track.id, track.name));
                ui.label(format!("Length: {:?}", track.length));
                // Additional UI elements for each track can be added here
            });
        }
    }
}
