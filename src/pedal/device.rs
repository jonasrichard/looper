use eframe::egui::{ComboBox, Context, Modal};

use crate::{audio::DeviceType, pedal::App};

pub fn choose_device(app: &mut App, ctx: &Context) {
    Modal::new("choose_device".into()).show(ctx, |ui| {
        ui.set_width(240.0);
        ui.heading("Choose device");
        ui.separator();

        ui.label("Input devices");

        let mut selected = String::new();

        ComboBox::from_label("Select input device").show_ui(ui, |ui| {
            for device in app
                .devices
                .iter()
                .filter(|d| d.device_type == DeviceType::Input)
            {
                ui.selectable_value(
                    &mut selected,
                    device.id.clone(),
                    device.description.clone(),
                );
            }
        });

        ui.label("Output devices");

        ComboBox::from_label("Select output device").show_ui(ui, |ui| {
            for device in app
                .devices
                .iter()
                .filter(|d| d.device_type == DeviceType::Output)
            {
                ui.selectable_value(
                    &mut selected,
                    device.id.clone(),
                    device.description.clone(),
                );
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Ok").clicked() {
                app.device_choose_open = false;
            }

            if ui.button("Cancel").clicked() {
                app.device_choose_open = false;
            }
        });
    });
}
