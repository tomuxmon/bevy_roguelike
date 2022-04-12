use super::Attributes;
use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Capability {
    // ActionPoints
    ap_turn_ready: u32,
    ap_current: u32,
    ap_increment: u32,
    // NOTE: HitPoints
    hp_max: i32,
    hp_current: i32,
    // NOTE: Attack
    attack_cost: u32,
    attack_damage: i32,
}
impl Capability {
    pub const AP_TURN_READY_DEFAULT: u32 = 1000;
    pub const AP_INCREMENT_MIN: u32 = 100;
    pub const HP_MAX_MIN: i32 = 500;
    pub const ATTACK_COST_MAX: u32 = 900;
    pub const ATTACK_DAMAGE_MIN: i32 = 50;

    pub fn new(attributes: Attributes) -> Self {
        let str = *attributes.get("strength").unwrap_or(&5);
        let tou = *attributes.get("toughness").unwrap_or(&5);
        let will = *attributes.get("willpower").unwrap_or(&5);
        let dex = *attributes.get("dexterity").unwrap_or(&5);

        let ap_increment = Capability::AP_INCREMENT_MIN + (dex * 5 + will * 3) as u32;
        let hp_max = Capability::HP_MAX_MIN + tou * 10 + str * 3 + will * 2 + dex;
        let attack_cost = Capability::ATTACK_COST_MAX - (dex * 10) as u32;
        let attack_damage = Capability::ATTACK_DAMAGE_MIN + str * 10 + dex * 3;

        Self {
            ap_turn_ready: Capability::AP_TURN_READY_DEFAULT,
            ap_increment,
            ap_current: 0,
            hp_max,
            hp_current: hp_max,
            attack_cost,
            attack_damage,
        }
    }

    pub fn ap_turn_ready_to_act(&self) -> u32 {
        self.ap_turn_ready
    }
    pub fn ap_current(&self) -> u32 {
        self.ap_current
    }
    pub fn ap_current_add(&mut self) -> u32 {
        self.ap_current = self.ap_current + self.ap_increment;
        self.ap_current
    }
    pub fn ap_current_add_delta(&mut self, time: &Time) -> u32 {
        let breath = (self.ap_increment as f32 * time.delta_seconds()) as u32;
        self.ap_current = self.ap_current + breath;
        self.ap_current
    }
    pub fn ap_current_minus(&mut self, cost: u32) -> u32 {
        self.ap_current -= cost;
        self.ap_current
    }
    pub fn hp_apply(&mut self, amount: i32) -> i32 {
        self.hp_current = i32::min(self.hp_current + amount, self.hp_max);
        self.hp_current
    }
    pub fn hp_current(&self) -> i32 {
        self.hp_current
    }
    pub fn attack_cost(&self) -> u32 {
        self.attack_cost
    }
    pub fn attack_damage(&self) -> i32 {
        self.attack_damage
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TurnState {
    Collect,
    Act,
    End,
}
impl Default for TurnState {
    fn default() -> Self {
        TurnState::Collect
    }
}
