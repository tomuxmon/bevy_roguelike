use crate::{
    components::*,
    resources::{Map, Tile},
};
use bevy::{prelude::*, utils::HashSet};
use line_drawing::{BresenhamCircle, WalkGrid};

pub fn field_of_view_set_visibility(
    fovs: Query<&FieldOfView, With<Player>>,
    visibles: Query<(&Vector2D, &Children, Option<&Behaviour>), With<VisibilityFOV>>,
    mut visible_children: Query<(&mut Sprite, &mut Visibility)>,
) {
    for fov in fovs.iter() {
        for (pt, clds, bh) in visibles.iter() {
            for c in clds.iter() {
                if let Ok((mut s, mut v)) = visible_children.get_mut(*c) {
                    let is_revealed = fov.tiles_revealed.contains(&pt);
                    let is_visible = fov.tiles_visible.contains(&pt);
                    v.is_visible = is_visible || (is_revealed && bh.is_none());
                    s.color = if is_visible && is_revealed {
                        Color::WHITE
                    } else {
                        Color::rgb(0.65, 0.65, 0.65)
                    };
                }
            }
        }
    }
}

pub fn field_of_view_recompute(mut actors: Query<(&Vector2D, &mut FieldOfView)>, map: Res<Map>) {
    // TODO: also include immediate environment (vec![Vector2D; 8] )

    actors
        .iter_mut()
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pt, mut fov)| {
            let last_visible = fov.tiles_visible.clone();
            fov.tiles_revealed.extend(last_visible);
            fov.tiles_visible = compute_fov(*pt, fov.radius, &*map);
            fov.is_dirty = false;
        });
}

fn compute_fov(pt: Vector2D, radius: i32, map: &Map) -> HashSet<Vector2D> {
    let mut fov = HashSet::default();
    for (xo, yo) in BresenhamCircle::new(pt.x(), pt.y(), radius) {
        for vpt in WalkGrid::new((pt.x(), pt.y()), (xo, yo))
            .map(|(x, y)| Vector2D::new(x, y))
            .filter(|p| map.is_in_bounds(*p))
        {
            if map[vpt] == Tile::Wall {
                fov.insert(vpt);
                break;
            }
            fov.insert(vpt);
        }
    }
    fov
}
