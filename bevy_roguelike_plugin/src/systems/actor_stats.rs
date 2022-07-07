use crate::components::*;
use bevy::prelude::*;
use bevy_inventory::{Equipment, ItemType};
use bevy_inventory_ui::UiTextInfo;
use bevy_roguelike_combat::{stats_derived::*, *};

#[allow(clippy::type_complexity)]
pub fn actors_fill_text_info(
    mut cmd: Commands,
    players: Query<&FieldOfView, With<MovingPlayer>>,
    actors: Query<(
        Entity,
        &Name,
        &Team,
        &ActionPoints,
        &HitPoints,
        &StatsComputed,
        &Vector2D,
        Option<&UiTextInfo>,
    )>,
) {
    for player_fov in players.iter() {
        for (actor_entity, name, team, ap, hp, stats, pt, info) in actors.iter() {
            if player_fov.tiles_visible.iter().any(|t| *t == **pt) {
                let mut titles_descriptions = vec![];
                titles_descriptions.push(("Team".to_string(), format!("{}", team.id())));
                titles_descriptions.push(("Speed".to_string(), format!("{}", ap.increment())));
                titles_descriptions.push(("Hit points".to_string(), hp.full().to_string()));
                titles_descriptions
                    .push(("Attributes".to_string(), format!("{}", stats.attributes)));
                cmd.entity(actor_entity).insert(UiTextInfo {
                    name: name.as_str().to_string(),
                    titles_descriptions,
                });
            } else if info.is_some() {
                cmd.entity(actor_entity).remove::<UiTextInfo>();
            }
        }
    }
}

pub fn attributes_update_field_of_view(
    mut cmd: Commands,
    mut actors: Query<(Entity, &StatsComputed, &mut FieldOfView), With<FieldOfViewDirty>>,
) {
    for (id, stats, mut fov) in actors.iter_mut() {
        fov.update(&stats.attributes);
        cmd.entity(id).remove::<FieldOfViewDirty>();
    }
}

#[allow(clippy::type_complexity)]
pub fn stats_recompute<I: ItemType>(
    mut cmd: Commands,
    mut actors: Query<
        (
            Entity,
            &mut StatsComputed,
            &Attributes,
            &Protection,
            &Resistance,
            &Evasion,
            &DamageList,
            &Equipment<I>,
        ),
        With<StatsComputedDirty>,
    >,
    items_atr: Query<&Attributes, (With<I>, Without<Vector2D>)>,
    items_prt: Query<&Protection, (With<I>, Without<Vector2D>)>,
    items_res: Query<&Resistance, (With<I>, Without<Vector2D>)>,
    items_blk: Query<&Block, (With<I>, Without<Vector2D>)>,
    items_dmg: Query<&Damage, (With<I>, Without<Vector2D>)>,
) {
    for (
        id,
        mut stats,
        innate_attributes,
        innate_protection,
        innate_resistance,
        evasion,
        unarmed_damage,
        equipment,
    ) in actors.iter_mut()
    {
        // NOTE: a lot of cloning, but hopefully not a common action to equip / unequip stuff
        stats.attributes =
            equipment.list(&items_atr).into_iter().sum::<Attributes>() + innate_attributes.clone();
        stats.protection = equipment
            .list(&items_prt)
            .iter()
            .fold(&mut Protection::default(), |acc, p| acc.extend(p))
            .extend(innate_protection)
            .clone();
        stats.resistance = equipment
            .list(&items_res)
            .iter()
            .fold(&mut Resistance::default(), |acc, p| acc.ingest(p))
            .ingest(innate_resistance)
            .clone();
        stats.evasion = evasion.clone();
        stats.block = equipment.list(&items_blk);
        let mut damage = equipment.list(&items_dmg);
        if damage.is_empty() {
            damage.extend(unarmed_damage.list.clone());
        }
        stats.damage = damage;

        cmd.entity(id)
            .remove::<StatsComputedDirty>()
            .insert(ActionPointsDirty {})
            .insert(HitPointsDirty {})
            .insert(FieldOfViewDirty {});
    }
}
