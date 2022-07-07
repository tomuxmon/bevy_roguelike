use crate::ActionPointsDirty;
use crate::HitPointsDirty;
use crate::IdleEvent;
use crate::events::AttackEvent;
use crate::events::DamageHitPointsEvent;
use crate::events::DeathEvent;
use crate::events::SpendAPEvent;
use crate::ActionPoints;
use crate::HitPoints;
use crate::StatsComputed;
use bevy::log;
use bevy::prelude::*;
use rand::prelude::*;

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

pub fn idle_rest(
    mut actors: Query<(&mut HitPoints, &ActionPoints)>,
    mut idle_reader: EventReader<IdleEvent>,
    mut ap_spend_writer: EventWriter<SpendAPEvent>,
) {
    for e in idle_reader.iter() {
        ap_spend_writer.send(SpendAPEvent::new(e.id, ActionPoints::IDLE_COST_DEFAULT));
        if let Ok((mut hp, ap)) = actors.get_mut(e.id) {
            let ratio = ActionPoints::IDLE_COST_DEFAULT as f32 / ap.turn_ready_to_act() as f32;
            hp.regen_ratio(ratio);
        }
    }
}

pub fn attack(
    attackers: Query<&StatsComputed>,
    defenders: Query<(&StatsComputed, &ActionPoints)>,
    mut attack_reader: EventReader<AttackEvent>,
    mut ap_spend_writer: EventWriter<SpendAPEvent>,
    mut damage_writer: EventWriter<DamageHitPointsEvent>,
    mut rng: ResMut<StdRng>,
) {
    for e in attack_reader.iter() {
        let attacker_stats = if let Ok(attacker) = attackers.get(e.attacker) {
            attacker
        } else {
            log::info!(
                "Attacker Not Found (id: {:?}). Probably died recently.",
                e.attacker
            );
            return;
        };
        let (defender_stats, defender_ap) = if let Ok(defender) = defenders.get(e.defender) {
            defender
        } else {
            log::info!(
                "Defender Not Found (id: {:?}). Probably died recently.",
                e.defender
            );
            return;
        };

        if attacker_stats.damage.is_empty() {
            log::error!("attacker has no damage.");
            return;
        }

        let rng = &mut *rng;

        let damage = &attacker_stats.damage[rng.gen_range(0..attacker_stats.damage.len())];

        // TODO: spawn attack animation (based on damage.kind)

        // NOTE: attacker should spend AP regardles of outcome
        let attack_cost = damage.hit_cost.compute(&attacker_stats.attributes);
        ap_spend_writer.send(SpendAPEvent::new(e.attacker, attack_cost));
        log::trace!("attacking with cost {}", attack_cost);

        // NOTE: negative AP is ok as long as we are close to zero (not reaching i16::MIN).
        if defender_ap.current() > 0 {
            let (evaded, evade_cost) = defender_stats.evasion.try_evade(
                damage,
                &defender_stats.attributes,
                &attacker_stats.attributes,
                rng,
            );

            if evaded {
                // TODO: spawn evade animation (MISS on the enemy)
                ap_spend_writer.send(SpendAPEvent::new(e.defender, evade_cost));
                log::trace!("attack evaded with cost {}", evade_cost);
                return;
            } else {
                // TODO: roll crit hit (hit rate vs evade rate)
            }

            for block in defender_stats.block.iter() {
                let (blocked, block_cost) = block.try_block(
                    damage,
                    &defender_stats.attributes,
                    &attacker_stats.attributes,
                    rng,
                );
                if blocked {
                    // TODO: spawn block animation
                    ap_spend_writer.send(SpendAPEvent::new(e.defender, block_cost));
                    log::trace!("attack blocked with cost {}", block_cost);
                    return;
                }
            }
        }

        let mut true_damage = damage.compute(&attacker_stats.attributes, rng);
        log::trace!(
            "attack damage raw {} (roll from {:?})",
            true_damage,
            damage.amount
        );

        // NOTE: apply protection and only then resistance
        for protect in defender_stats
            .protection
            .amounts
            .iter()
            .filter(|p| p.kind == damage.kind)
        {
            true_damage -= protect.compute(&defender_stats.attributes);
        }
        if true_damage < 1 {
            log::trace!(
                "damage negated with protection. damage after protection {}",
                true_damage
            );
            // TODO: spawn clinc animation
            return;
        }

        let resist = defender_stats
            .resistance
            .amounts
            .iter()
            .filter(|r| r.kind == damage.kind)
            .map(|r| r.percent)
            .sum::<u8>()
            .min(100) as f32
            / 100.;

        true_damage = (true_damage as f32 * (1. - resist)) as i32;

        damage_writer.send(DamageHitPointsEvent {
            defender: e.defender,
            amount: -true_damage as i16,
        });

        log::trace!("attack damage {}", true_damage);
    }
}

pub fn apply_damage_hit_points(
    mut actors: Query<&mut HitPoints>,
    mut damage_reader: EventReader<DamageHitPointsEvent>,
    mut death_writer: EventWriter<DeathEvent>,
) {
    for e in damage_reader.iter() {
        if let Ok(mut hp) = actors.get_mut(e.defender) {
            hp.apply(e.amount);
            if hp.current() <= 0 {
                death_writer.send(DeathEvent { actor: e.defender });
            }
        }
    }
}



