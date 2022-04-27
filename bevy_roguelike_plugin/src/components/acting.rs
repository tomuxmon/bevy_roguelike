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
    // NOTE: hp regen
    hp_regen_ready: i32,
    hp_regen_current: i32,
    hp_regen_increment: i32,
    // NOTE: Attack
    attack_cost: i32,
    attack_damage: i32,
}
impl Capability {
    pub const DELTA_COST_MOVE_DEFAULT: i32 = 100;
    pub const IDLE_COST_DEFAULT: i32 = 64;

    pub const AP_TURN_READY_DEFAULT: i32 = 128;
    pub const HP_REGEN_READY_DEFAULT: i32 = 128;

    pub const AP_INCREMENT_MIN: i32 = 64;
    pub const HP_MAX_MIN: i32 = 20;
    pub const ATTACK_COST_MAX: i32 = 128;
    pub const ATTACK_COST_MIN: i32 = 36;
    pub const ATTACK_DAMAGE_MIN: i32 = 1;
    pub const HP_REGEN_INCREMENT_MIN: i32 = 64;

    pub fn new(attributes: Attributes) -> Self {
        let str = *attributes.get("strength").unwrap_or(&5);
        let tou = *attributes.get("toughness").unwrap_or(&5);
        let will = *attributes.get("willpower").unwrap_or(&5);
        let dex = *attributes.get("dexterity").unwrap_or(&5);

        let ap_increment = Capability::AP_INCREMENT_MIN + dex * 7 + will * 3;
        let hp_max = Capability::HP_MAX_MIN + tou * 6 + str * 2 + will;
        let attack_cost = i32::max(
            Capability::ATTACK_COST_MAX - dex * 4,
            Capability::ATTACK_COST_MIN,
        );
        let attack_damage = Capability::ATTACK_DAMAGE_MIN + str;
        let hp_regen_ready = Capability::HP_REGEN_READY_DEFAULT;
        let hp_regen_current = 0;
        let hp_regen_increment = Capability::HP_REGEN_INCREMENT_MIN + tou * 4 + str * 2 + will;

        Self {
            ap_turn_ready: Capability::AP_TURN_READY_DEFAULT,
            ap_increment,
            ap_current: 0,
            hp_max,
            hp_current: hp_max,
            hp_regen_ready,
            hp_regen_current,
            hp_regen_increment,
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
    pub fn regen(&mut self) {
        self.hp_regen_current = self.hp_regen_current + self.hp_regen_increment;
        if self.hp_regen_current > self.hp_regen_ready {
            let amount = self.hp_regen_current / self.hp_regen_ready;
            let rem = self.hp_regen_current % self.hp_regen_ready;
            self.hp_apply(amount);
            self.hp_regen_current = rem;
        }
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
