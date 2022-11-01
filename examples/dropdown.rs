use eframe::{App, Frame, NativeOptions};
use egui::{Context, Widget};
use egui_dropdown::DropDownBox;

struct ExampleApp {
    items: Vec<String>,
    buf: String,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                DropDownBox::from_iter(&self.items, "test_dropbox", &mut self.buf, |ui, text| {
                    ui.selectable_label(false, text)
                })
                .ui(ui);

                if ui.button("Add").clicked() {
                    self.items.push(self.buf.clone());
                }
            });
        });
    }
}

fn main() {
    eframe::run_native(
        "egui-dropdown",
        NativeOptions::default(),
        Box::new(|_| {
            Box::new(ExampleApp {
                items: vec![
                    "First".into(),
                    "Second".into(),
                    "Third".into(),
                    "Other".into(),
                ],
                buf: String::new(),
            })
        }),
    );
}
