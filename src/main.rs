use bevy::prelude::*;
use bevy_roguelike_plugin::{
    components::*, events::*, resources::*, systems::turns::gather_action_points, AssetsLoading,
    RoguelikePlugin, StateNext,
};
use line_drawing::WalkGrid;
use map_generator::*;
use rand::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Setup,
    AssetLoad,
    Construct,
    InGame,
    Pause,
    Reseting,
}

impl StateNext for AppState {
    fn next(&self) -> Option<Self> {
        match self {
            AppState::Setup => Some(AppState::AssetLoad),
            AppState::AssetLoad => Some(AppState::Construct),
            AppState::Construct => Some(AppState::InGame),
            AppState::InGame => Some(AppState::Pause),
            AppState::Pause => Some(AppState::InGame),
            AppState::Reseting => Some(AppState::Construct),
        }
    }
}

pub fn input_player(
    keys: Res<Input<KeyCode>>,
    players: Query<(Entity, &TurnState, &Inventory, &Equipment), With<MovingPlayer>>,
    mut act_writer: EventWriter<ActEvent>,
    mut pick_up_writer: EventWriter<PickUpItemEvent>,
    mut drop_writer: EventWriter<DropItemEvent>,
) {
    for (id, _, inv, eqv) in players
        .iter()
        .filter(|(_, ts, _, _)| **ts == TurnState::Act)
    {
        let delta = if keys.just_pressed(KeyCode::Up) {
            IVec2::new(0, 1)
        } else if keys.just_pressed(KeyCode::Down) {
            IVec2::new(0, -1)
        } else if keys.just_pressed(KeyCode::Left) {
            IVec2::new(-1, 0)
        } else if keys.just_pressed(KeyCode::Right) {
            IVec2::new(1, 0)
        } else if keys.pressed(KeyCode::Space) {
            IVec2::new(0, 0) // stay put - skip turn
        } else if keys.pressed(KeyCode::Comma) {
            pick_up_writer.send(PickUpItemEvent::new(id));
            IVec2::new(0, 0) // still stay put - skip turn
        } else if keys.just_pressed(KeyCode::D) {
            if let Some(ee) = inv.iter_some().last() {
                drop_writer.send(DropItemEvent::new(id, ee));
            } else {
                if let Some((_, ee)) = eqv.iter_some().last() {
                    drop_writer.send(DropItemEvent::new(id, ee));
                }
            }
            return;
        } else {
            return;
        };
        act_writer.send(ActEvent::new(id, delta));
    }
}

pub fn input_fov_rand(
    mut rng: ResMut<StdRng>,
    actors: Query<(Entity, &Vector2D, &Team, &TurnState, &FieldOfView), With<MovingFovRandom>>,
    mut act_writer: EventWriter<ActEvent>,
    team_map: Res<TeamMap>,
    map: Res<Map>,
) {
    for (id, pt, team, _, fov) in actors
        .iter()
        .filter(|(_, _, _, ts, _)| **ts == TurnState::Act)
    {
        let deltas = vec![
            IVec2::new(0, 1),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(0, 0), // stay put - skip turn
            IVec2::new(0, 0), // stay put - skip turn
            IVec2::new(0, 0), // stay put - skip turn
            IVec2::new(0, 0), // stay put - skip turn
            IVec2::new(0, 0), // stay put - skip turn
        ];

        // NOTE: closest oposing team member search
        let mut distance_last = ((fov.radius + 1) * (fov.radius + 1)) as f32;
        let mut pt_move_target: Option<IVec2> = None;
        for pt_visible in fov.tiles_visible.iter() {
            if let Some(other_team) = team_map[*pt_visible] {
                if other_team != *team {
                    let distance = pt_visible.as_vec2().distance_squared(pt.as_vec2());
                    if distance < distance_last {
                        pt_move_target = Some(*pt_visible);
                        distance_last = distance;
                    }
                }
            }
        }

        let mut delta = IVec2::new(0, 0);
        if let Some(tgt) = pt_move_target {
            if let Some((x, y)) = WalkGrid::new((pt.x, pt.y), (tgt.x, tgt.y)).take(2).last() {
                let dest = IVec2::new(x, y);
                if map[dest] == Tile::Floor
                    && !(team_map[dest].is_some() && team_map[dest].unwrap() == *team)
                {
                    delta = IVec2::new(x - pt.x, y - pt.y);
                }
            }
        } else {
            delta = deltas[rng.gen_range(0..deltas.len())];
        }
        act_writer.send(ActEvent::new(id, delta));
    }
}

fn main() {
    let mut app = App::new();
    app.add_state(AppState::Setup)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "rogue bevy".to_string(),
            width: 1200.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins {})
        .add_plugin(RoguelikePlugin {
            asset_load_state: AppState::AssetLoad,
            game_construct_state: AppState::Construct,
            running_state: AppState::InGame,
        })
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(input_player.after(gather_action_points))
                .with_system(input_fov_rand.after(gather_action_points)),
        )
        .add_startup_system(rogue_setup);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn rogue_setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
    mut loading: ResMut<AssetsLoading>,
) {
    cmd.insert_resource(MapOptions {
        map_size: IVec2::new(20, 15),
        tile_size: 32.0,
    });

    let map_assets = MapAssets {
        floor: vec![
            asset_server.load("sprites/floor/pebble_brown_0.png"),
            asset_server.load("sprites/floor/pebble_brown_1.png"),
            asset_server.load("sprites/floor/pebble_brown_2.png"),
            asset_server.load("sprites/floor/pebble_brown_3.png"),
            asset_server.load("sprites/floor/pebble_brown_4.png"),
            asset_server.load("sprites/floor/pebble_brown_5.png"),
            asset_server.load("sprites/floor/pebble_brown_6.png"),
            asset_server.load("sprites/floor/pebble_brown_7.png"),
            asset_server.load("sprites/floor/pebble_brown_8.png"),
        ],
        wall: vec![
            asset_server.load("sprites/walls/brick_brown_0.png"),
            asset_server.load("sprites/walls/brick_brown_1.png"),
            asset_server.load("sprites/walls/brick_brown_2.png"),
            asset_server.load("sprites/walls/brick_brown_3.png"),
            asset_server.load("sprites/walls/brick_brown_4.png"),
            asset_server.load("sprites/walls/brick_brown_5.png"),
            asset_server.load("sprites/walls/brick_brown_6.png"),
            asset_server.load("sprites/walls/brick_brown_7.png"),
        ],
    };
    loading
        .0
        .extend(map_assets.floor.iter().map(|a| a.clone_untyped()));
    loading
        .0
        .extend(map_assets.wall.iter().map(|a| a.clone_untyped()));
    cmd.insert_resource(map_assets);

    let player_assets = PlayerAssets {
        body: asset_server.load("sprites/player/human_male.png"),
        wear: vec![
            asset_server.load("sprites/player/jacket_2.png"),
            asset_server.load("sprites/player/pants_black.png"),
        ],
    };
    loading
        .0
        .extend(player_assets.wear.iter().map(|a| a.clone_untyped()));
    loading.0.push(player_assets.body.clone_untyped());
    cmd.insert_resource(player_assets);

    let enemy_assets = EnemyAssets {
        skins: vec![
            asset_server.load("sprites/enemy/cyclops.png"),
            asset_server.load("sprites/enemy/ettin.png"),
            asset_server.load("sprites/enemy/frost_giant.png"),
            asset_server.load("sprites/enemy/gnoll.png"),
            asset_server.load("sprites/enemy/goblin.png"),
            asset_server.load("sprites/enemy/hobgoblin.png"),
            asset_server.load("sprites/enemy/kobold.png"),
            asset_server.load("sprites/enemy/orc.png"),
            asset_server.load("sprites/enemy/stone_giant.png"),
        ],
    };
    loading
        .0
        .extend(enemy_assets.skins.iter().map(|a| a.clone_untyped()));
    cmd.insert_resource(enemy_assets);

    let item_assets = ItemAssets {
        skins: vec![
            asset_server.load("sprites/item/buckler_1.png"),
            asset_server.load("sprites/item/club.png"),
            asset_server.load("sprites/item/gold_green.png"),
            asset_server.load("sprites/item/orcish_dagger.png"),
            asset_server.load("sprites/item/ring_mail_1.png"),
            asset_server.load("sprites/item/spear.png"),
            asset_server.load("sprites/item/two_handed_sword.png"),
        ],
    };
    loading
        .0
        .extend(item_assets.skins.iter().map(|a| a.clone_untyped()));
    cmd.insert_resource(item_assets);

    let inventory_assets = InventoryAssets {
        slot: asset_server.load("sprites/gui/inventory/slot.png"),
        head_wear: asset_server.load("sprites/gui/inventory/head_wear.png"),
        body_wear: asset_server.load("sprites/gui/inventory/body_wear.png"),
        main_hand_gear: asset_server.load("sprites/gui/inventory/main_hand_gear.png"),
        off_hand_gear: asset_server.load("sprites/gui/inventory/off_hand_gear.png"),
        finger_wear: asset_server.load("sprites/gui/inventory/finger_wear.png"),
        neck_wear: asset_server.load("sprites/gui/inventory/neck_wear.png"),
        feet_wear: asset_server.load("sprites/gui/inventory/feet_wear.png"),
    };
    loading.0.push(inventory_assets.slot.clone_untyped());
    loading.0.push(inventory_assets.head_wear.clone_untyped());
    loading.0.push(inventory_assets.body_wear.clone_untyped());
    loading
        .0
        .push(inventory_assets.main_hand_gear.clone_untyped());
    loading
        .0
        .push(inventory_assets.off_hand_gear.clone_untyped());
    loading.0.push(inventory_assets.finger_wear.clone_untyped());
    loading.0.push(inventory_assets.neck_wear.clone_untyped());
    loading.0.push(inventory_assets.feet_wear.clone_untyped());
    cmd.insert_resource(inventory_assets);

    let prefabs = Prefabs {
        ron_scene: asset_server.load("prefabs/prefabs.scn.ron"),
    };
    loading.0.push(prefabs.ron_scene.clone_untyped());
    cmd.insert_resource(prefabs);

    state.set(AppState::AssetLoad).unwrap();
}
