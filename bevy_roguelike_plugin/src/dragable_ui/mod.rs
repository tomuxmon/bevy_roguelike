use bevy::prelude::*;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Drag {
    is_started: bool,
}

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct DragTracker {
    pub last_ui_position: Rect<Val>,
    pub last_cursor_position: Vec2,
}

#[derive(Debug, Copy, Clone)]
pub struct DragCursorMoved {
    pub id: Entity,
    pub just_started: bool,
    pub current_position: Vec2,
}

impl DragCursorMoved {
    pub fn new(id: Entity, just_started: bool, current_position: Vec2) -> Self {
        Self {
            id,
            just_started,
            current_position,
        }
    }
}

pub fn ui_drag_interaction(
    mut cursor_moved_reader: EventReader<CursorMoved>,
    mut drag_cursor_moved_writer: EventWriter<DragCursorMoved>,
    mut interactive_dragables: Query<(Entity, &Interaction, &mut Drag)>,
) {
    for mm in cursor_moved_reader.iter() {
        for (entity, i, mut d) in interactive_dragables.iter_mut() {
            let mut just_started = false;
            if d.is_started {
                if *i != Interaction::Clicked {
                    d.is_started = false;
                }
            } else {
                if *i == Interaction::Clicked {
                    d.is_started = true;
                    just_started = true;
                }
            }
            if d.is_started {
                drag_cursor_moved_writer.send(DragCursorMoved::new(
                    entity,
                    just_started,
                    mm.position,
                ));
            }
        }
    }
}

pub fn ui_apply_drag_pos(
    mut drag_cursor_moved_reader: EventReader<DragCursorMoved>,
    mut interactive_dragables: Query<
        (&mut Style, &mut DragTracker),
        (With<Interaction>, With<Drag>),
    >,
) {
    for e in drag_cursor_moved_reader.iter() {
        if let Ok((mut style, mut track_pos)) = interactive_dragables.get_mut(e.id) {
            if e.just_started {
                *track_pos = DragTracker {
                    last_ui_position: style.position.clone(),
                    last_cursor_position: e.current_position,
                };
            }
            let delta = track_pos.last_cursor_position - e.current_position;

            let top = if let Val::Px(i) = track_pos.last_ui_position.top {
                i
            } else {
                0.
            };
            let right = if let Val::Px(i) = track_pos.last_ui_position.right {
                i
            } else {
                0.
            };

            style.position.top = Val::Px(top + delta.y);
            style.position.right = Val::Px(right + delta.x);
        }
    }
}
