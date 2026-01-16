use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::prelude::*;

use super::{warn_no_editor_on_picking_event, InputState};

/// Whenever a pointer enters a widget
#[derive(Message, Debug, Reflect)]
pub struct TextHoverIn;

/// Whenever a pointer exits a widget
#[derive(Message, Debug, Reflect)]
pub struct TextHoverOut;

/// What cursor icon to show when hovering over a widget
///
/// By default is [`CursorIcon::System(SystemCursorIcon::Text)`]
#[derive(Component, Reflect, Deref)]
pub struct HoverCursor(pub CursorIcon);

impl Default for HoverCursor {
    fn default() -> Self {
        Self(CursorIcon::System(SystemCursorIcon::Text))
    }
}

impl InputState {
    /// `Over` event handler
    pub fn start_hovering(&mut self) {
        trace!("Starting hover");
        match self {
            InputState::Idle => *self = InputState::Hovering,
            InputState::Hovering | InputState::Dragging { .. } => {}
        }
    }

    pub fn is_hovering(&self) -> bool {
        matches!(self, InputState::Hovering)
    }

    /// Handler for [`Move`] event
    pub fn continue_hovering(&mut self) {
        match self {
            InputState::Hovering | InputState::Dragging { .. } => {}
            InputState::Idle => {
                // handles that case that a drag is finished
                *self = InputState::Hovering;
            }
        }
    }

    /// `Out` event handler
    pub fn end_hovering(&mut self) {
        trace!("Ending hoverr");
        match self {
            InputState::Hovering => *self = InputState::Idle,
            InputState::Idle | InputState::Dragging { .. } => {}
        }
    }
}

pub(super) fn handle_hover_start(
    event: On<Pointer<Over>>,
    mut editor: Query<&mut InputState, With<CosmicEditBuffer>>,
    mut hover_in_evw: MessageWriter<TextHoverIn>,
) {
    let Ok(mut input_state) = editor.get_mut(event.entity) else {
        warn_no_editor_on_picking_event("handling cursor `Over` event");
        return;
    };

    input_state.start_hovering();

    if input_state.is_hovering() {
        hover_in_evw.write(TextHoverIn);
    }
}

pub(super) fn handle_hover_continue(
    event: On<Pointer<Move>>,
    mut editor: Query<&mut InputState, With<CosmicEditBuffer>>,
) {
    let Ok(mut input_state) = editor.get_mut(event.entity) else {
        warn_no_editor_on_picking_event("handling cursor `Move` event");
        return;
    };

    input_state.continue_hovering();
}

pub(super) fn handle_hover_end(
    event: On<Pointer<Out>>,
    mut editor: Query<&mut InputState, With<CosmicEditBuffer>>,
    mut hover_out_evw: MessageWriter<TextHoverOut>,
) {
    let Ok(mut input_state) = editor.get_mut(event.entity) else {
        warn_no_editor_on_picking_event("handling cursor `Out` event");
        return;
    };

    input_state.end_hovering();

    if !input_state.is_hovering() {
        hover_out_evw.write(TextHoverOut);
    }
}
