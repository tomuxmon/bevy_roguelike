use crate::{
    components::*,
    resources::{Map, Tile},
};
use bevy::{prelude::*, tasks::*, utils::HashSet};
use line_drawing::{BresenhamCircle, Supercover};

pub fn field_of_view_set_visibility_info(
    pool: Res<AsyncComputeTaskPool>,
    players: Query<&FieldOfView, With<Player>>,
    mut visibles: Query<(
        &Vector2D,
        &Children,
        Option<&Behaviour>,
        &mut VisibilityToggle,
    )>,
) {
    for fov in players.iter() {
        visibles.par_for_each_mut(&*pool, 16, |(pt, clds, bh, mut vt)| {
            for c in clds.iter() {
                // TODO: rewrite with no inserts. just updates.
                let is_revealed = fov.tiles_revealed.contains(&pt);
                let is_visible = fov.tiles_visible.contains(&pt);
                let is_ambient = bh.is_none();
                vt.insert(*c, VisibilityInfo::new(is_revealed, is_visible, is_ambient));
            }
        });
    }
}

pub fn field_of_view_set_visivility(
    visibles: Query<&VisibilityToggle>,
    mut visible_children: Query<(&mut Sprite, &mut Visibility)>,
) {
    // still no paralelism :|
    for vt in visibles.iter() {
        for (e, i) in vt.iter() {
            if let Ok((mut s, mut v)) = visible_children.get_mut(*e) {
                v.is_visible = i.is_visible || (i.is_revealed && i.is_ambient);
                s.color = if i.is_visible && i.is_revealed {
                    Color::WHITE
                } else {
                    Color::rgb(0.65, 0.65, 0.65)
                };
            }
        }
    }
}

pub fn field_of_view_recompute(
    pool: Res<AsyncComputeTaskPool>,
    mut actors: Query<(&Vector2D, &mut FieldOfView)>,
    map: Res<Map>,
) {
    actors.par_for_each_mut(&*pool, 16, |(pt, mut fov)| {
        if !fov.is_dirty {
            return;
        }
        let visible_last = fov.tiles_visible.clone();
        fov.tiles_visible = compute_fov(*pt, fov.radius, &*map);
        let visible_current = fov.tiles_visible.clone();
        fov.tiles_revealed.extend(visible_last);
        fov.tiles_revealed.extend(visible_current);
        fov.is_dirty = false;
    });
}

fn compute_fov(pt: Vector2D, radius: i32, map: &Map) -> HashSet<Vector2D> {
    let mut fov = HashSet::default();
    for (xo, yo) in BresenhamCircle::new(pt.x(), pt.y(), radius) {
        for vpt in Supercover::new((pt.x(), pt.y()), (xo, yo))
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
