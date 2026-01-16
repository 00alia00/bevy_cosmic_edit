use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use cosmic_text::Action;

use crate::{prelude::*, ScrollEnabled};

pub(crate) fn scroll(
    mut editor: Query<(&mut CosmicEditor, &ScrollEnabled)>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut scroll_evr: MessageReader<MouseWheel>,
) {
    let font_system = &mut font_system.0;
    for (mut editor, scroll_enabled) in editor.iter_mut() {
        let mut editor = editor.borrow_with(font_system);

        if **scroll_enabled {
            for ev in scroll_evr.read() {
                match ev.unit {
                    MouseScrollUnit::Line => {
                        // trace!(?ev, "Line");
                        let line_height = editor.with_buffer(|b| b.metrics().line_height);
                        editor.action(Action::Scroll {
                            pixels: -ev.y * line_height,
                        });
                    }
                    MouseScrollUnit::Pixel => {
                        // trace!(?ev, "Pixel");
                        editor.action(Action::Scroll {
                            pixels: -ev.y,
                        });
                    }
                }
            }
        }
    }
}
