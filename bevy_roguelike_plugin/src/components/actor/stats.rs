use bevy::prelude::*;

#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct Attributes {
    pub strength: u8,
    pub dexterity: u8,
    pub inteligence: u8,
    pub toughness: u8,
    pub perception: u8,
    pub willpower: u8,
}
impl Attributes {
    pub fn new(
        strength: u8,
        dexterity: u8,
        inteligence: u8,
        toughness: u8,
        perception: u8,
        willpower: u8,
    ) -> Self {
        Self {
            strength,
            dexterity,
            inteligence,
            toughness,
            perception,
            willpower,
        }
    }
}

// TODO: split up into separate components
// ActionCounter,
// HitPointCounter
// HitPointRegenCounter
// AttackStatsCounter
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Capability {
    // NOTE: ActionPoints
    ap_turn_ready: i16,
    ap_current: i16,
    ap_increment: i16,
    // NOTE: HitPoints
    hp_max: i16,
    hp_current: i16,
    // NOTE: hp regen
    hp_regen_ready: i16,
    hp_regen_current: i16,
    hp_regen_increment: i16,
    // NOTE: Attack
    attack_cost: i16,
    attack_damage: i16,
}
impl Capability {
    pub const DELTA_COST_MOVE_DEFAULT: i16 = 100;
    pub const IDLE_COST_DEFAULT: i16 = 64;

    pub const AP_TURN_READY_DEFAULT: i16 = 128;
    pub const HP_REGEN_READY_DEFAULT: i16 = 128;

    pub const AP_INCREMENT_MIN: i16 = 64;
    pub const HP_MAX_MIN: i16 = 20;
    pub const ATTACK_COST_MAX: i16 = 128;
    pub const ATTACK_COST_MIN: i16 = 36;
    pub const ATTACK_DAMAGE_MIN: i16 = 1;
    pub const HP_REGEN_INCREMENT_MIN: i16 = 64;

    pub fn new(attributes: Attributes) -> Self {
        let str = attributes.strength as i16;
        let tou = attributes.toughness as i16;
        let will = attributes.willpower as i16;
        let dex = attributes.dexterity as i16;

        let ap_increment = Capability::AP_INCREMENT_MIN + dex * 7 + will * 3;
        let hp_max = Capability::HP_MAX_MIN + tou * 6 + str * 2 + will;
        let attack_cost = i16::max(
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

    pub fn ap_turn_ready_to_act(&self) -> i16 {
        self.ap_turn_ready
    }
    pub fn ap_current(&self) -> i16 {
        self.ap_current
    }
    pub fn ap_current_add(&mut self) -> i16 {
        self.ap_current = self.ap_current + self.ap_increment;
        self.ap_current
    }
    pub fn ap_current_add_delta(&mut self, time: &Time) -> i16 {
        let breath = (self.ap_increment as f32 * time.delta_seconds()) as i16;
        self.ap_current = self.ap_current + breath;
        self.ap_current
    }
    pub fn ap_current_minus(&mut self, cost: i16) -> i16 {
        self.ap_current -= cost;
        self.ap_current
    }
    pub fn hp_apply(&mut self, amount: i16) -> i16 {
        self.hp_current = i16::min(self.hp_current + amount, self.hp_max);
        self.hp_current
    }
    pub fn hp_current(&self) -> i16 {
        self.hp_current
    }
    pub fn hp_percent(&self) -> u8 {
        (self.hp_current * 100 / self.hp_max) as u8
    }
    pub fn attack_cost(&self) -> i16 {
        self.attack_cost
    }
    pub fn attack_damage(&self) -> i16 {
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