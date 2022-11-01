use eframe::{App, Frame, NativeOptions};
use egui::Context;
use egui_dropdown::DropDownBox;

#[derive(Default)]
struct ExampleApp {
    items: Vec<String>,
    buf: String,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(DropDownBox::from_iter(
                    &self.items,
                    "test_dropbox",
                    &mut self.buf,
                ));

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
        Box::new(|_| Box::new(ExampleApp::default())),
    );
}
