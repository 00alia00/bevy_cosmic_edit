//! Manages the OS-level cursor aka mouse pointer visibility

use bevy::input::mouse::MouseMotion;
use bevy::window::CursorOptions;
use bevy::{ecs::system::SystemParam, window::PrimaryWindow};

use crate::prelude::*;

use crate::input::CosmicTextChanged;

#[derive(SystemParam)]
pub(crate) struct CursorVisibility<'w, 's> {
    cursor_options: Single<'w, 's, &'static mut CursorOptions, With<PrimaryWindow>>,
}

impl CursorVisibility<'_, '_> {
    pub fn set_cursor_visibility(&mut self, visible: bool) {
        self.cursor_options.visible = visible;
    }
}

pub(super) fn update_cursor_visibility(
    editors_text_changed: MessageReader<CosmicTextChanged>,
    mouse_moved: MessageReader<MouseMotion>,
    mouse_clicked: Res<ButtonInput<MouseButton>>,
    mut cursor_visibility: CursorVisibility,
) {
    let text_changed_at_all = !editors_text_changed.is_empty();
    if text_changed_at_all {
        cursor_visibility.set_cursor_visibility(false);
    }

    let mouse_moved_at_all = !mouse_moved.is_empty();
    let mouse_clicked_at_all = mouse_clicked.get_just_pressed().len() != 0;
    if mouse_moved_at_all || mouse_clicked_at_all {
        cursor_visibility.set_cursor_visibility(true);
    }
}
