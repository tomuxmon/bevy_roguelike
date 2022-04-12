use crate::{
    components::*,
    resources::{Map, Tile},
};
use bevy::{prelude::*, utils::HashSet};
use line_drawing::{BresenhamCircle, WalkGrid};

pub fn field_of_view_set_visibility(
    fovs: Query<&FieldOfView, With<Player>>,
    visibles: Query<(&Vector2D, &Children), With<VisibilityFOV>>,
    // use &Sprite, for shades of darkness
    mut visible_children: Query<&mut Visibility>,
) {
    for fov in fovs.iter() {
        for (pt, clds) in visibles.iter() {
            for c in clds.iter() {
                if let Ok(mut v) = visible_children.get_mut(*c) {
                    v.is_visible = fov.visible_tiles.contains(&pt);
                }
            }
        }
    }
}

pub fn field_of_view_recompute(mut actors: Query<(&Vector2D, &mut FieldOfView)>, map: Res<Map>) {
    actors
        .iter_mut()
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pt, mut fov)| {
            fov.visible_tiles = compute_fov(*pt, fov.radius, &*map);
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
