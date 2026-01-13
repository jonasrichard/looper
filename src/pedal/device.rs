use eframe::egui::{Context, Modal, Ui};

use crate::pedal::App;

pub fn choose_device(app: &mut App, ctx: &Context) {
    Modal::new("choose_device".into()).show(ctx, |ui| {
        ui.set_width(240.0);
        ui.heading("Choose device");
        ui.label("Test");
        if ui.button("Cancel").clicked() {
            app.device_choose_open = false;
        }
    });
}
