use super::Attributes;
use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Capability {
    // NOTE: ActionPoints
    ap_turn_ready: i32,
    ap_current: i32,
    ap_increment: i32,
    // NOTE: HitPoints
    hp_max: i32,
    hp_current: i32,
    // NOTE: Attack
    attack_cost: i32,
    attack_damage: i32,
}
impl Capability {
    pub const AP_TURN_READY_DEFAULT: i32 = 1024;
    pub const AP_INCREMENT_MIN: i32 = 1024;
    pub const HP_MAX_MIN: i32 = 500;
    pub const ATTACK_COST_MAX: i32 = 900;
    pub const ATTACK_DAMAGE_MIN: i32 = 50;

    pub fn new(attributes: Attributes) -> Self {
        let str = *attributes.get("strength").unwrap_or(&5);
        let tou = *attributes.get("toughness").unwrap_or(&5);
        let will = *attributes.get("willpower").unwrap_or(&5);
        let dex = *attributes.get("dexterity").unwrap_or(&5);

        let ap_increment = Capability::AP_INCREMENT_MIN + dex * 8 + will * 4;
        let hp_max = Capability::HP_MAX_MIN + tou * 10 + str * 3 + will * 2 + dex;
        let attack_cost = Capability::ATTACK_COST_MAX - dex * 10;
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

    pub fn ap_turn_ready_to_act(&self) -> i32 {
        self.ap_turn_ready
    }
    pub fn ap_current(&self) -> i32 {
        self.ap_current
    }
    pub fn ap_current_add(&mut self) -> i32 {
        self.ap_current = self.ap_current + self.ap_increment;
        self.ap_current
    }
    pub fn ap_current_add_delta(&mut self, time: &Time) -> i32 {
        let breath = (self.ap_increment as f32 * time.delta_seconds()) as i32;
        self.ap_current = self.ap_current + breath;
        self.ap_current
    }
    pub fn ap_current_minus(&mut self, cost: i32) -> i32 {
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
    pub fn hp_percent(&self) -> u8 {
        (self.hp_current * 100 / self.hp_max) as u8
    }
    pub fn attack_cost(&self) -> i32 {
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

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ModifyHP {
    pub location: IVec2,
    pub amount: i32,
}

impl ModifyHP {
    pub fn new(location: IVec2, amount: i32) -> Self {
        Self { location, amount }
    }
}
