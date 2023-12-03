//! egui-dropdown

#![warn(missing_docs)]

use egui::{Id, Response, TextEdit, Ui, Widget, WidgetText};
use std::hash::Hash;

/// Dropdown widget
pub struct DropDownBox<
    'a,
    F: FnMut(&mut Ui, &str) -> Response,
    V: AsRef<str>,
    I: Iterator<Item = V>,
> {
    buf: &'a mut String,
    popup_id: Id,
    display: F,
    it: I,
    hint_text: WidgetText,
}

impl<'a, F: FnMut(&mut Ui, &str) -> Response, V: AsRef<str>, I: Iterator<Item = V>>
    DropDownBox<'a, F, V, I>
{
    /// Creates new dropdown box.
    pub fn from_iter(
        it: impl IntoIterator<IntoIter = I>,
        id_source: impl Hash,
        buf: &'a mut String,
        display: F,
    ) -> Self {
        Self {
            popup_id: Id::new(id_source),
            it: it.into_iter(),
            display,
            buf,
            hint_text: WidgetText::default(),
        }
    }

    /// Add a hint text to the Text Edit
    pub fn hint_text(mut self, hint_text: impl Into<WidgetText>) -> Self {
        self.hint_text = hint_text.into();
        self
    }
}

impl<'a, F: FnMut(&mut Ui, &str) -> Response, V: AsRef<str>, I: Iterator<Item = V>> Widget
    for DropDownBox<'a, F, V, I>
{
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            popup_id,
            buf,
            it,
            mut display,
            hint_text,
        } = self;

        let mut r = ui.add(TextEdit::singleline(buf).hint_text(hint_text));
        if r.gained_focus() {
            ui.memory_mut(|m| m.open_popup(popup_id));
        }

        let mut changed = false;
        egui::popup_below_widget(ui, popup_id, &r, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for var in it {
                    let text = var.as_ref();
                    if !buf.is_empty() && !text.to_lowercase().contains(&buf.to_lowercase()) {
                        continue;
                    }

                    if display(ui, text).clicked() {
                        *buf = text.to_owned();
                        changed = true;

                        ui.memory_mut(|m| m.close_popup());
                    }
                }
            });
        });

        if changed {
            r.mark_changed();
        }

        r
    }
}
