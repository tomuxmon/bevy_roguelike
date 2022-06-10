use bevy::prelude::*;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct DragableUI {
    is_started: bool,
    current_cursor_position: Vec2,
    last_ui_position: Rect<Val>,
    last_cursor_position: Vec2,
}

pub fn ui_drag_interaction(
    mut cursor_moved_reader: EventReader<CursorMoved>,
    mut interactive_dragables: Query<(&Interaction, &Style, &mut DragableUI)>,
) {
    for mm in cursor_moved_reader.iter() {
        for (i, s, mut d) in interactive_dragables.iter_mut() {
            if d.is_started {
                if *i != Interaction::Clicked {
                    d.is_started = false;
                    d.last_ui_position = Rect::default();
                    d.last_cursor_position = Vec2::ZERO;
                    d.current_cursor_position = Vec2::ZERO;
                }
            } else if *i == Interaction::Clicked {
                d.is_started = true;
                d.last_ui_position = s.position;
                d.last_cursor_position = mm.position;
            }
            if d.is_started {
                d.current_cursor_position = mm.position;
            }
        }
    }
}

pub fn ui_apply_drag_pos(mut dragables: Query<(&mut Style, &DragableUI)>) {
    for (mut style, d) in dragables.iter_mut().filter(|(_, d)| d.is_started) {
        let delta = d.last_cursor_position - d.current_cursor_position;
        let top = if let Val::Px(i) = d.last_ui_position.top {
            i
        } else {
            0.
        };
        let right = if let Val::Px(i) = d.last_ui_position.right {
            i
        } else {
            0.
        };
        style.position.top = Val::Px(top + delta.y);
        style.position.right = Val::Px(right + delta.x);
        // update z?
    }
}
