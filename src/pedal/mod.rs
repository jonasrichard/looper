use eframe::{
    egui::{
        self, CentralPanel, Context, FontId, SidePanel, TextStyle,
        TopBottomPanel,
    },
    run_native,
};

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    ) {
        set_styles(ctx);
        set_top_bar(ctx);

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("A");
                ui.label("B");
            });
        });

        SidePanel::right("right_panel").show(ctx, |ui| {});

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
    // TODO set min sizes, increase initial size
    let options = eframe::NativeOptions {
        viewport: eframe::egui::viewport::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([320.0, 240.0]),
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

fn set_top_bar(ctx: &Context) {
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::containers::menu::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.menu_button(
                "Audio",
                |ui| if ui.button("Devices...").clicked() {},
            );
        });
    });
}
