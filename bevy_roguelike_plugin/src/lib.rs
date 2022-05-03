pub mod components;
pub mod events;
pub mod map_generator;
pub mod resources;
pub mod systems;

use crate::components::*;
use crate::events::*;
use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
use bevy_easings::*;
use map_generator::*;
use rand::prelude::*;
use resources::*;
use systems::actor_stats::*;
use systems::camera::*;
use systems::fov::*;
use systems::item::*;
use systems::map::*;
use systems::render::*;
use systems::turns::*;

pub struct RoguelikePlugin<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for RoguelikePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(EasingsPlugin {})
            .add_startup_system(camera_setup)
            .add_system_set(
                SystemSet::on_enter(self.running_state.clone()).with_system(Self::create_map),
            )
            .add_system_set(
                SystemSet::on_update(self.running_state.clone())
                    .with_system(render_body)
                    .with_system(render_hud_health_bar)
                    .with_system(attributes_update_action_points)
                    .with_system(attributes_update_hit_points)
                    .with_system(attributes_update_attack_stats)
                    .with_system(attributes_update_defense_stats)
                    .with_system(attributes_update_field_of_view)
                    .with_system(gather_action_points)
                    .with_system(turn_end_now_gather.after(gather_action_points))
                    .with_system(act)
                    .with_system(attack.after(act))
                    .with_system(pick_up_items)
                    .with_system(drop_item)
                    .with_system(spend_ap.after(act))
                    .with_system(do_move.after(act).after(spend_ap))
                    .with_system(apply_position_to_transform.after(do_move))
                    .with_system(apply_hp_modify.after(act).after(spend_ap))
                    .with_system(idle_rest.after(apply_hp_modify))
                    .with_system(camera_set_focus_player)
                    .with_system(camera_focus_smooth.after(camera_set_focus_player))
                    .with_system(field_of_view_recompute)
                    .with_system(field_of_view_set_visibility.after(field_of_view_recompute)),
            )
            .add_system_set(
                SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_map),
            )
            .register_type::<Vector2D>()
            .register_type::<RenderInfo>()
            .register_type::<MapTile>()
            .register_type::<Attributes>()
            .register_type::<ActionPoints>()
            .register_type::<AttackStats>()
            .register_type::<DefenseStats>()
            .register_type::<HitPoints>()
            .register_type::<TurnState>()
            .register_type::<ModifyHP>()
            .register_type::<Team>()
            .register_type::<MovingPlayer>()
            .register_type::<MovingRandom>()
            .register_type::<MovingFovRandom>()
            .register_type::<FieldOfView>()
            .register_type::<Item>()
            .register_type::<AttackBoost>()
            .register_type::<DefenseBoost>()
            .register_type::<Equiped>()
            .add_event::<SpendAPEvent>()
            .add_event::<AttackEvent>()
            .add_event::<MoveEvent>()
            .add_event::<ActEvent>()
            .add_event::<IdleEvent>()
            .add_event::<PickUpItemEvent>()
            .add_event::<DropItemEvent>()
            .add_event::<CameraFocusEvent>();

        log::info!("Loaded Roguelike Plugin");
    }
}

#[derive(Debug)]
pub struct MapEntities {
    map_id: Entity,
    enemies_id: Entity,
}

impl<T> RoguelikePlugin<T> {
    fn cleanup_map(map_id: Res<MapEntities>, mut cmd: Commands) {
        cmd.entity(map_id.map_id).despawn_recursive();
        cmd.entity(map_id.enemies_id).despawn_recursive();
        cmd.remove_resource::<MapEntities>();
    }

    pub fn create_map(
        mut cmd: Commands,
        map_options: Option<Res<MapOptions>>,
        map_assets: Res<MapAssets>,
        player_assets: Res<PlayerAssets>,
        enemy_assets: Res<EnemyAssets>,
        item_assets: Res<ItemAssets>,
        mut cameras: Query<&mut Transform, With<Camera>>,
    ) {
        let options = match map_options {
            None => MapOptions::default(), // If no options is set we use the default one
            Some(o) => o.clone(),
        };

        // max u64: 18_446_744_073_709_551_615
        // let mut rng = StdRng::seed_from_u64(54155745465);
        let trng = thread_rng();
        let mut rng = StdRng::from_rng(trng).expect("Could not construct StdRng using ThreadRng");

        let map_generator = RandomMapGenerator {};
        let map = map_generator.gen(&mut rng, options.map_size);
        let info = MapInfo::from_map(&map, &mut rng);
        let mut team_map = TeamMap::empty(options.map_size);

        #[cfg(feature = "debug")]
        log::info!("{}", map.to_colorized_string());
        #[cfg(feature = "debug")]
        log::info!("{}", info.to_colorized_string());

        for mut c in cameras.iter_mut() {
            let z = c.translation.z;
            let new_pos = options.to_world_position(info.camera_focus).extend(z);
            c.translation = new_pos;
        }

        let map_id = cmd
            .spawn()
            .insert(Name::new("RogueMap"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|rogue_map| {
                for (pt, tile) in map.enumerate() {
                    let texture = match tile {
                        Tile::Wall => {
                            map_assets.wall[rng.gen_range(0..map_assets.wall.len())].clone()
                        }
                        Tile::Floor => {
                            map_assets.floor[rng.gen_range(0..map_assets.floor.len())].clone()
                        }
                    };
                    rogue_map
                        .spawn()
                        .insert(Name::new(format!("Tile {}", pt)))
                        .insert(Vector2D::from(pt))
                        .insert(match tile {
                            Tile::Wall => MapTile { is_passable: false },
                            Tile::Floor => MapTile { is_passable: true },
                        })
                        .insert(RenderInfo { texture, z: 0. });
                }
            })
            .id();

        for ipt in info.item_spawns.clone() {
            if rng.gen_bool(0.5) {
                let damage = rng.gen_range(1..16);
                let rate = rng.gen_range(1..16);
                let cost = rng.gen_range(4..16);
                cmd.spawn()
                    .insert_bundle(Weapon::new(
                        "weapon",
                        AttackBoost::new(damage, rate, cost),
                        item_assets.skins[rng.gen_range(0..item_assets.skins.len())].clone(),
                    ))
                    .insert(Vector2D::from(ipt));
            } else {
                let absorb = rng.gen_range(1..8);
                let rate = rng.gen_range(1..8);
                let cost = rng.gen_range(4..12);
                cmd.spawn()
                    .insert_bundle(Armor::new(
                        "armor",
                        DefenseBoost::new(absorb, rate, cost),
                        item_assets.skins[rng.gen_range(0..item_assets.skins.len())].clone(),
                    ))
                    .insert(Vector2D::from(ipt));
            }
        }

        let plr_atr = Attributes::new(11, 11, 11, 11, 11, 11);
        let team_player = 1;
        cmd.spawn()
            .insert(MovingPlayer {})
            .insert_bundle(Actor::new(
                "Player",
                team_player,
                plr_atr,
                info.player_start,
                player_assets.body.clone(),
            ))
            .with_children(|player| {
                // TODO: instead should be equiped inventory
                spawn_player_body_wear(player, &player_assets, options.tile_size);
            });

        team_map[info.player_start] = Some(Team::new(team_player));

        let enemies_id = cmd
            .spawn()
            .insert(Name::new("Enemies"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|enms| {
                for mpt in info.monster_spawns.clone() {
                    let mon_atr = Attributes::new(
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                    );

                    let team_monster = 1 + rng.gen_range(2..4);

                    enms.spawn()
                        .insert(MovingFovRandom {})
                        .insert_bundle(Actor::new(
                            "Enemy",
                            team_monster,
                            mon_atr,
                            mpt,
                            enemy_assets.skins[rng.gen_range(0..enemy_assets.skins.len())].clone(),
                        ));

                    team_map[mpt] = Some(Team::new(team_monster));
                }
            })
            .id();

        cmd.insert_resource(map);
        cmd.insert_resource(info);
        cmd.insert_resource(rng);
        cmd.insert_resource(team_map);
        cmd.insert_resource(MapEntities { map_id, enemies_id });
    }
}

fn spawn_player_body_wear(cb: &mut ChildBuilder, player_assets: &PlayerAssets, size: f32) {
    for i in 0..player_assets.wear.len() {
        cb.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            texture: player_assets.wear[i].clone(),
            transform: Transform::from_xyz(0., 0., 4.),
            ..Default::default()
        });
    }
}

fn camera_setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}
