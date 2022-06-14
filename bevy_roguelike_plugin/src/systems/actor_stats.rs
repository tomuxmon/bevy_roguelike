use crate::components::*;
use bevy::prelude::*;
use bevy_inventory::{Equipment, ItemType};

pub fn attributes_update_action_points(
    mut cmd: Commands,
    mut actors: Query<(Entity, &StatsComputed, &mut ActionPoints), With<ActionPointsDirty>>,
) {
    for (id, stats, mut ap) in actors.iter_mut() {
        ap.update(&stats.attributes);
        cmd.entity(id).remove::<ActionPointsDirty>();
    }
}
pub fn attributes_update_hit_points(
    mut cmd: Commands,
    mut actors: Query<(Entity, &StatsComputed, &mut HitPoints), With<HitPointsDirty>>,
) {
    for (id, stats, mut hp) in actors.iter_mut() {
        hp.update(&stats.attributes);
        cmd.entity(id).remove::<HitPointsDirty>();
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
