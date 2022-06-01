use crate::components::*;
use bevy::prelude::*;

// TODO: macro for a repeating code
// TODO: trait for a repeating update(Attributes) function
// TODO: investigate why Changed<_> does not work as intended (includes non changes)

pub fn attributes_update_action_points(
    mut actors: Query<(&Attributes, &mut ActionPoints, Changed<Attributes>)>,
) {
    for (atr, mut ap, _) in actors.iter_mut() {
        ap.update(atr);
    }
}
pub fn attributes_update_hit_points(
    mut actors: Query<(&Attributes, &mut HitPoints, Changed<Attributes>)>,
) {
    for (atr, mut hp, _) in actors.iter_mut() {
        hp.update(atr);
    }
}
pub fn attributes_update_field_of_view(
    mut actors: Query<(&Attributes, &mut FieldOfView, Changed<Attributes>)>,
) {
    for (atr, mut fov, _) in actors.iter_mut() {
        fov.update(atr);
    }
}

pub fn stats_recompute(
    mut actors: Query<(
        &mut StatsComputed,
        &Attributes,
        &Protection,
        &Resistance,
        &Evasion,
        &Damage,
        &Equipment,
    )>,
    items_atr: Query<&Attributes, (With<ItemType>, Without<Vector2D>)>,
    items_prt: Query<&Protection, (With<ItemType>, Without<Vector2D>)>,
    items_res: Query<&Resistance, (With<ItemType>, Without<Vector2D>)>,
    items_blk: Query<&Block, (With<ItemType>, Without<Vector2D>)>,
    items_dmg: Query<&Damage, (With<ItemType>, Without<Vector2D>)>,
) {
    for (
        mut stats,
        innate_attributes,
        innate_protection,
        innate_resistance,
        evasion,
        unarmed_damage,
        equipment,
    ) in actors.iter_mut()
    {
        if !stats.is_updated {
            // NOTE: a lot of cloning, but hopefully not a common action to equip / unequip stuff
            stats.attributes = equipment.list(&items_atr).into_iter().sum::<Attributes>()
                + innate_attributes.clone();
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
            if damage.len() == 0 {
                damage.push(unarmed_damage.clone());
            }
            stats.damage = damage;
            stats.is_updated = true;
        }
    }
}
