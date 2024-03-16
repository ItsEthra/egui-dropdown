use eframe::{App, Frame, NativeOptions};
use egui::Context;
use egui_dropdown::DropDownBox;

struct ExampleApp {
    items: Vec<String>,
    buf: String,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    DropDownBox::from_iter(
                        &self.items,
                        "test_dropbox",
                        &mut self.buf,
                        |ui, text| ui.selectable_label(false, text),
                    )
                    // choose wether to filter the box items based on what is in the text edit already
                    // default is true when this is not used
                    .filter_by_input(false)
                    // choose wether to select all text in the text edit when it gets focused
                    // default is false when this is not used
                    .select_on_focus(true),
                );

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
    )
    .unwrap();
}
