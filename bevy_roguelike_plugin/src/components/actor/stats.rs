use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
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

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ActionPoints {
    turn_ready: i16,
    current: i16,
    increment: i16,
}
impl ActionPoints {
    pub const DELTA_COST_MOVE_DEFAULT: i16 = 100;
    pub const IDLE_COST_DEFAULT: i16 = 64;
    pub const TURN_READY_DEFAULT: i16 = 128;
    pub const INCREMENT_MIN: i16 = 64;

    pub fn new(atr: &Attributes) -> Self {
        let will = atr.willpower as i16;
        let dex = atr.dexterity as i16;
        let increment = ActionPoints::INCREMENT_MIN + dex * 7 + will * 3;
        Self {
            turn_ready: ActionPoints::TURN_READY_DEFAULT,
            increment,
            current: 0,
        }
    }

    pub fn turn_ready_to_act(&self) -> i16 {
        self.turn_ready
    }
    pub fn current(&self) -> i16 {
        self.current
    }
    pub fn current_add(&mut self) -> i16 {
        self.current = self.current + self.increment;
        self.current
    }
    pub fn current_minus(&mut self, cost: i16) -> i16 {
        self.current -= cost;
        self.current
    }
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct HitPoints {
    max: i16,
    current: i16,

    regen_ready: i16,
    regen_current: i16,
    regen_increment: i16,
}
impl HitPoints {
    pub const MAX_MIN: i16 = 20;
    pub const REGEN_READY_DEFAULT: i16 = 128;
    pub const REGEN_INCREMENT_MIN: i16 = 64;

    pub fn new(atr: &Attributes) -> Self {
        let str = atr.strength as i16;
        let tou = atr.toughness as i16;
        let will = atr.willpower as i16;

        let max = HitPoints::MAX_MIN + tou * 6 + str * 2 + will;
        let regen_ready = HitPoints::REGEN_READY_DEFAULT;
        let regen_current = 0;
        let regen_increment = HitPoints::REGEN_INCREMENT_MIN + tou * 4 + str * 2 + will;

        Self {
            max,
            current: max,
            regen_ready,
            regen_current,
            regen_increment,
        }
    }

    pub fn apply(&mut self, amount: i16) -> i16 {
        self.current = i16::min(self.current + amount, self.max);
        self.current
    }
    pub fn current(&self) -> i16 {
        self.current
    }
    pub fn percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
    pub fn regen(&mut self) {
        self.regen_current = self.regen_current + self.regen_increment;
        if self.regen_current > self.regen_ready {
            let amount = self.regen_current / self.regen_ready;
            let rem = self.regen_current % self.regen_ready;
            self.apply(amount);
            self.regen_current = rem;
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct AttackStats {
    cost: i16,
    damage: i16,
}
impl AttackStats {
    pub const COST_MAX: i16 = 128;
    pub const COST_MIN: i16 = 36;
    pub const DAMAGE_MIN: i16 = 1;

    pub fn new(atr: &Attributes) -> Self {
        let str = atr.strength as i16;
        let dex = atr.dexterity as i16;

        let attack_cost = i16::max(AttackStats::COST_MAX - dex * 4, AttackStats::COST_MIN);
        let attack_damage = AttackStats::DAMAGE_MIN + str;

        Self {
            cost: attack_cost,
            damage: attack_damage,
        }
    }
    pub fn cost(&self) -> i16 {
        self.cost
    }
    pub fn damage(&self) -> i16 {
        self.damage
    }
}
