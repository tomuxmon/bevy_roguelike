use crate::{components::*, map_generator::*, resources::MapOptions};
use bevy::{prelude::*, tasks::*, utils::HashSet};
use line_drawing::{BresenhamCircle, Supercover};

pub fn field_of_view_set_visibility(
    players: Query<&FieldOfView, With<MovingPlayer>>,
    mut visibles: Query<(
        &Vector2D,
        &Children,
        Option<&ActionPoints>,
        Option<&HitPoints>,
        &VisibilityToggle,
    )>,
    mut visible_children: Query<(
        &mut Sprite,
        &mut Transform,
        &mut Visibility,
        Option<&OnTopHud>,
    )>,
    map_options: Res<MapOptions>,
) {
    for fov in players.iter() {
        visibles.for_each_mut(|(pt, children, cp, hp, _)| {
            let is_revealed = fov.tiles_revealed.contains(&pt);
            let is_visible = fov.tiles_visible.contains(&pt);
            let is_ambient = cp.is_none();
            let hp_percent = if hp.is_none() {
                1.
            } else {
                hp.unwrap().percent()
            };
            for c in children.iter() {
                if let Ok((mut s, mut t, mut v, h)) = visible_children.get_mut(*c) {
                    let is_hud = h.is_some();
                    v.is_visible = (is_visible && !is_hud)
                        || (is_visible && is_hud && hp_percent != 1.)
                        || (is_ambient && is_revealed);

                    if !is_hud {
                        s.color = if is_visible && is_revealed {
                            Color::WHITE
                        } else {
                            Color::rgb(0.65, 0.65, 0.65)
                        };
                    } else {
                        s.color.set_g(hp_percent);
                        s.color.set_r(1. - hp_percent);

                        if let Some(size) = s.custom_size {
                            let x = map_options.tile_size * hp_percent;
                            let slide = map_options.tile_size - x;
                            s.custom_size = Some(Vec2::new(x, size.y));
                            t.translation.x = -slide / 2.;
                        }
                    }
                }
            }
        });
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
        fov.tiles_visible = compute_fov(**pt, fov.radius, &*map);
        let visible_current = fov.tiles_visible.clone();
        fov.tiles_revealed.extend(visible_last);
        fov.tiles_revealed.extend(visible_current);
        fov.is_dirty = false;
    });
}

fn compute_fov(pt: IVec2, radius: i32, map: &Map) -> HashSet<IVec2> {
    let mut fov = HashSet::default();
    for (xo, yo) in BresenhamCircle::new(pt.x, pt.y, radius) {
        for vpt in Supercover::new((pt.x, pt.y), (xo, yo))
            .map(|(x, y)| IVec2::new(x, y))
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
