pub mod components;
pub mod dragable_ui;
pub mod events;
pub mod quality;
pub mod resources;
pub mod systems;

use crate::components::*;
use crate::dragable_ui::*;
use crate::events::*;
use crate::quality::*;
use bevy::ecs::schedule::StateData;
use bevy::ecs::system::EntityCommands;
use bevy::log;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;
use bevy::utils::HashSet;
use bevy_asset_ron::RonAssetPlugin;
use bevy_easings::*;
use map_generator::*;
use rand::prelude::*;
use resources::*;
use std::ops::Range;
use systems::actor_stats::*;
use systems::camera::*;
use systems::fov::*;
use systems::inventory::*;
use systems::map::*;
use systems::render::*;
use systems::turns::*;

pub struct RoguelikePlugin<T> {
    /// Asset loading happens in this state. When it finishes it transitions to [`RoguelikePlugin::game_construct_state`]
    pub asset_load_state: T,
    pub game_construct_state: T,
    pub running_state: T,
}

pub trait StateNext: StateData {
    fn next(&self) -> Option<Self>;
}
pub trait StateSetNext {
    fn set_next(&mut self);
}
impl<T: StateNext> StateSetNext for State<T> {
    fn set_next(&mut self) {
        let current = self.current();
        if let Some(next) = current.next() {
            if let Err(error) = self.set(next) {
                bevy::log::error!("Error setting next state. {}", error);
            }
        } else {
            bevy::log::error!("no next state for {:?}.", current);
        }
    }
}

#[derive(Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

#[derive(Debug)]
pub struct MapEntities {
    map_id: Entity,
    enemies_id: Entity,
}

impl<T: StateNext> Plugin for RoguelikePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(EasingsPlugin {})
            .add_plugin(RonAssetPlugin::<ItemTemplate>::new(&["item.ron"]))
            .insert_resource(AssetsLoading::default())
            .add_startup_system(setup_camera)
            .add_system_set(
                SystemSet::on_update(self.asset_load_state.clone())
                    .with_system(Self::check_assets_ready),
            )
            .add_system_set(
                SystemSet::on_enter(self.game_construct_state.clone())
                    .with_system(Self::create_map),
            )
            .add_system_to_stage(CoreStage::First, ui_drag_interaction)
            .add_system_set(
                SystemSet::on_update(self.running_state.clone())
                    .with_system(ui_apply_drag_pos)
                    .with_system(render_body)
                    .with_system(render_hud_health_bar)
                    .with_system(attributes_update_action_points)
                    .with_system(attributes_update_hit_points)
                    .with_system(attributes_update_field_of_view)
                    .with_system(gather_action_points)
                    .with_system(turn_end_now_gather.after(gather_action_points))
                    .with_system(act)
                    .with_system(attack.after(act))
                    .with_system(pick_up_items)
                    .with_system(toggle_inventory_open)
                    .with_system(equipment_update)
                    .with_system(inventory_update)
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
            .register_type::<RenderInfoEquiped>()
            .register_type::<MapTile>()
            .register_type::<Attributes>()
            .register_type::<AttributeType>()
            .register_type::<ActionPoints>()
            .register_type::<HitPoints>()
            .register_type::<TurnState>()
            .register_type::<ModifyHP>()
            .register_type::<Team>()
            .register_type::<MovingPlayer>()
            .register_type::<MovingRandom>()
            .register_type::<MovingFovRandom>()
            .register_type::<FieldOfView>()
            .register_type::<DamageKind>()
            .register_type::<AttributeMultiplier>()
            .register_type::<Formula>()
            .register_type::<Rate>()
            .register_type::<ActionCost>()
            .register_type::<Damage>()
            .register_type::<Protect>()
            .register_type::<Resist>()
            .register_type::<Protection>()
            .register_type::<Resistance>()
            .register_type::<Evasion>()
            .register_type::<Block>()
            .register_type::<Evasion>()
            .register_type::<ItemType>()
            .register_type::<Quality>()
            .register_type::<ItemEquipSlot>()
            .register_type::<ItemDisplaySlot>()
            .register_type::<EquipmentDisplay>()
            .register_type::<Equipment>()
            .register_type::<DragableUI>()
            .register_type::<HashSet<IVec2>>()
            .register_type::<Vec<DamageKind>>()
            .register_type::<Vec<Protect>>()
            .register_type::<HashSet<Resist>>()
            .register_type::<Range<i32>>()
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

impl<T: StateNext> RoguelikePlugin<T> {
    fn check_assets_ready(
        mut commands: Commands,
        server: Res<AssetServer>,
        loading: Res<AssetsLoading>,
        mut state: ResMut<State<T>>,
    ) {
        use bevy::asset::LoadState;

        match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
            LoadState::Failed => {
                bevy::log::error!("Asset load failed. Check preceding warnings for more info. Transitioning to next state anyway (feeling adventurous).");
                state.set_next();
            }
            LoadState::Loaded => {
                state.set_next();
                commands.remove_resource::<AssetsLoading>();
                // (note: if you don't have any other handles to the assets
                // elsewhere, they will get unloaded after this)
            }
            _ => {
                bevy::log::info!("loading assets...");
            }
        }
    }

    fn cleanup_map(map_id: Res<MapEntities>, mut cmd: Commands) {
        cmd.entity(map_id.map_id).despawn_recursive();
        cmd.entity(map_id.enemies_id).despawn_recursive();
        cmd.remove_resource::<MapEntities>();
    }

    pub fn create_map(
        mut cmd: Commands,
        mut state: ResMut<State<T>>,
        map_options: Option<Res<MapOptions>>,
        map_assets: Res<MapAssets>,
        player_assets: Res<PlayerAssets>,
        enemy_assets: Res<EnemyAssets>,
        asset_server: Res<AssetServer>,
        item_templates: Res<Assets<ItemTemplate>>,
        mut cameras: Query<&mut Transform, With<Camera2d>>,
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

        // let aaa = ItemTemplate::Shield(Shield {
        //     name: "buckler".to_string(),
        //     render: ItemRenderInfo {
        //         render_path: "buckler_1.png".to_string(),
        //         render_equiped_path: Some("buckler_round_3.png".to_string()),
        //     },
        //     protection: Protection {
        //         amounts: vec![
        //             Protect {
        //                 kind: DamageKind::Blunt,
        //                 amount_multiplier: None,
        //                 amount: 2,
        //             },
        //             Protect {
        //                 kind: DamageKind::Pierce,
        //                 amount_multiplier: None,
        //                 amount: 2,
        //             },
        //             Protect {
        //                 kind: DamageKind::Slash,
        //                 amount_multiplier: None,
        //                 amount: 2,
        //             },
        //         ],
        //     },
        //     block: Block {
        //         block_type: vec![DamageKind::Blunt, DamageKind::Pierce, DamageKind::Slash],
        //         cost: ActionCost {
        //             cost: 45,
        //             multiplier_inverted: Formula::new(vec![AttributeMultiplier {
        //                 multiplier: 128,
        //                 attribute: AttributeType::Dexterity,
        //             }]),
        //         },
        //         chance: Rate {
        //             amount: 50,
        //             multiplier: Formula::new(vec![AttributeMultiplier {
        //                 multiplier: 100,
        //                 attribute: AttributeType::Dexterity,
        //             }]),
        //         },
        //     },
        // });

        // let my_config = ron::ser::PrettyConfig::new()
        //     .depth_limit(4)
        //     .indentor(" ".to_owned());
        // if let Ok(ron_str) = ron::ser::to_string_pretty(&aaa, my_config) {
        //     info!("ron: {}", ron_str);
        // } else {
        //     bevy::log::error!("no ron string huh..");
        // }

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

        let item_templates: Vec<_> = item_templates.iter().map(|(_, it)| it).collect();
        for ipt in info.item_spawns.clone() {
            let template = item_templates[rng.gen_range(0..item_templates.len())];
            let quality = Quality::roll(&mut rng);
            let mut ecmd = cmd.spawn();
            ecmd.insert(Vector2D::from(ipt));
            spawn_item(
                &mut ecmd,
                asset_server.clone(),
                template,
                &quality,
                &mut rng,
            );
        }

        let plr_atr = Attributes::new(11, 11, 11, 11, 11, 11);
        // DefaultAttack
        // DefaultDefense
        let team_player = 1;
        cmd.spawn()
            .insert(MovingPlayer {})
            .insert_bundle(Actor::new(
                "Player",
                team_player,
                &plr_atr,
                info.player_start,
                player_assets.body.clone(),
                get_player_equipment_slots(),
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
                            &mon_atr,
                            mpt,
                            enemy_assets.skins[rng.gen_range(0..enemy_assets.skins.len())].clone(),
                            vec![(
                                ItemType::MainHand,
                                0,
                                Rect {
                                    top: Val::Px(58.),
                                    left: Val::Px(72.),
                                    ..default()
                                },
                            )],
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

        state.set_next();
    }
}

fn spawn_item(
    ecmd: &mut EntityCommands,
    asset_server: AssetServer,
    template: &ItemTemplate,
    quality: &Quality,
    rng: &mut StdRng,
) {
    match template {
        ItemTemplate::Weapon(Weapon { render, damage }) => {
            ecmd.insert(ItemType::MainHand);
            insert_render(ecmd, asset_server, render, quality);
            ecmd.insert(damage.mutate(quality, rng));
        }
        ItemTemplate::Shield(Shield {
            render,
            protection,
            block,
        }) => {
            ecmd.insert(ItemType::OffHand);
            insert_render(ecmd, asset_server, render, quality);
            ecmd.insert(protection.mutate(quality, rng))
                .insert(block.mutate(quality, rng));
        }
        ItemTemplate::Helm(Helm {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(ItemType::Head);
            insert_render(ecmd, asset_server, render, quality);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
        ItemTemplate::Armor(Armor {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(ItemType::Body);
            insert_render(ecmd, asset_server, render, quality);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
        ItemTemplate::Boots(Boots {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(ItemType::Feet);
            insert_render(ecmd, asset_server, render, quality);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
        ItemTemplate::Amulet(Amulet {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(ItemType::Neck);
            insert_render(ecmd, asset_server, render, quality);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
        ItemTemplate::Ring(Ring {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(ItemType::Finger);
            insert_render(ecmd, asset_server, render, quality);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
    }
}

fn insert_defense(
    ecmd: &mut EntityCommands,
    defense: &ItemDefense,
    quality: &Quality,
    rng: &mut StdRng,
) {
    if let Some(prot) = defense.protection.clone() {
        ecmd.insert(prot.mutate(quality, rng));
    }
    if let Some(res) = defense.resistance.clone() {
        ecmd.insert(res.mutate(quality, rng));
    }
}
fn insert_enchantment(
    ecmd: &mut EntityCommands,
    enchantment: &ItemEnchantment,
    quality: &Quality,
    rng: &mut StdRng,
) {
    if let Some(attributes) = enchantment.attributes.clone() {
        ecmd.insert(attributes.mutate(quality, rng));
    }
}

fn insert_render(
    ecmd: &mut EntityCommands,
    asset_server: AssetServer,
    render: &ItemRenderInfo,
    quality: &Quality,
) {
    ecmd.insert(Name::new(format!("{} {}", quality, render.name.clone())))
        .insert(quality.clone())
        .insert(RenderInfo {
            texture: asset_server.load(render.texture_path.as_str()),
            z: 1.,
        });
    if let Some(path_equiped) = render.texture_equiped_path.clone() {
        ecmd.insert(RenderInfoEquiped {
            texture: asset_server.load(path_equiped.as_str()),
            z: 4.,
        });
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

fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmd.spawn_bundle(UiCameraBundle::default());
}

pub fn get_player_equipment_slots() -> Vec<(ItemType, u8, Rect<Val>)> {
    vec![
        (
            ItemType::MainHand,
            0,
            Rect {
                top: Val::Px(64. - 16.),
                left: Val::Px(128. - 32. - 16. - 8.),
                ..default()
            },
        ),
        (
            ItemType::OffHand,
            0,
            Rect {
                top: Val::Px(64. - 16.),
                left: Val::Px(128. + 32. - 16. + 8.),
                ..default()
            },
        ),
        (
            ItemType::Head,
            0,
            Rect {
                top: Val::Px(32. - 16. - 8.),
                left: Val::Px(128. - 16.),
                ..default()
            },
        ),
        (
            ItemType::Neck,
            0,
            Rect {
                top: Val::Px(32. - 16. - 8.),
                left: Val::Px(128. + 32. - 16. + 8.),
                ..default()
            },
        ),
        (
            ItemType::Body,
            0,
            Rect {
                top: Val::Px(64. - 16.),
                left: Val::Px(128. - 16.),
                ..default()
            },
        ),
        (
            ItemType::Feet,
            0,
            Rect {
                top: Val::Px(96. - 16. + 8.),
                left: Val::Px(128. - 16.),
                ..default()
            },
        ),
        (
            ItemType::Finger,
            0,
            Rect {
                top: Val::Px(96. - 16. + 8.),
                left: Val::Px(128. - 32. - 16. - 8.),
                ..default()
            },
        ),
        (
            ItemType::Finger,
            1,
            Rect {
                top: Val::Px(96. - 16. + 8.),
                left: Val::Px(128. + 32. - 16. + 8.),
                ..default()
            },
        ),
    ]
}
