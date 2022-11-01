//! egui-dropdown

#![warn(missing_docs)]

use egui::{Id, Response, Ui, Widget};
use std::hash::Hash;

/// Dropdown widget
pub struct DropDownBox<'a, V: AsRef<str>, I: Iterator<Item = V>> {
    buf: &'a mut String,
    popup_id: Id,
    it: I,
}

impl<'a, V: AsRef<str>, I: Iterator<Item = V>> DropDownBox<'a, V, I> {
    /// Creates new dropdown box.
    pub fn from_iter(
        it: impl IntoIterator<IntoIter = I>,
        id_source: impl Hash,
        buf: &'a mut String,
    ) -> Self {
        Self {
            popup_id: Id::new(id_source),
            it: it.into_iter(),
            buf,
        }
    }
}

impl<'a, V: AsRef<str>, I: Iterator<Item = V>> Widget for DropDownBox<'a, V, I> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { popup_id, buf, it } = self;

        let mut r = ui.text_edit_singleline(buf);
        if r.gained_focus() {
            ui.memory().open_popup(popup_id);
        }

        let mut changed = false;
        egui::popup_below_widget(ui, popup_id, &r, |ui| {
            for var in it {
                let text = var.as_ref();
                if !buf.is_empty() && !text.contains(&*buf) {
                    continue;
                }

                if ui.selectable_label(false, text).clicked() {
                    *buf = text.to_owned();
                    changed = true;

                    ui.memory().close_popup();
                }
            }
        });

        if changed {
            r.mark_changed();
        }

        r
    }
}
