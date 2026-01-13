use eframe::{
    egui::{CentralPanel, Context, FontId, SidePanel, TextStyle},
    run_native,
};

mod device;
mod menu;

#[derive(Default)]
struct App {
    device_choose_open: bool,
    input_devices: Vec<InputDevice>,
}

#[derive(Default)]
struct InputDevice {
    id: String,
}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        set_styles(ctx);
        menu::set_top_bar(self, ctx);

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("A");
                ui.label("B");
            });
        });

        SidePanel::right("right_panel").show(ctx, |_ui| {});

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label("Left controls");
                ui.label("Workspace");
                ui.label("Right controls");
            });
        });
    }
}

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::viewport::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([640.0, 360.0])
            .with_min_inner_size([480.0, 240.0]),
        ..Default::default()
    };

    run_native(
        "Dashboard",
        options,
        Box::new(|_| Ok(Box::<App>::default())),
    )
    .unwrap();
}

fn set_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(24.0, eframe::egui::FontFamily::Monospace),
        ),
        (
            TextStyle::Body,
            FontId::new(14.0, eframe::egui::FontFamily::Monospace),
        ),
        (
            TextStyle::Button,
            FontId::new(16.0, eframe::egui::FontFamily::Monospace),
        ),
        (
            TextStyle::Small,
            FontId::new(12.0, eframe::egui::FontFamily::Monospace),
        ),
    ]
    .into();

    ctx.set_style(style);
}
