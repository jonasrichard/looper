use eframe::egui::{self, Context, TopBottomPanel};

use crate::pedal::{App, device};

pub fn set_top_bar(app: &mut App, ctx: &Context) {
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::containers::menu::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.menu_button("Audio", |ui| {
                if ui.button("Devices...").clicked() {
                    app.device_choose_open = true;
                }
            });
        });
    });

    if app.device_choose_open {
        device::choose_device(app, ctx);
    }
}
