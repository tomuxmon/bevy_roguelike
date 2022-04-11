use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ActionPoints {
    turn_ready_ap: u32,
    current: u32,
    pub increment: u32,
}
impl ActionPoints {
    pub const DEFAULT_TURN_READY_AP: u32 = 1000;

    pub fn new(increment: u32) -> Self {
        Self {
            turn_ready_ap: ActionPoints::DEFAULT_TURN_READY_AP,
            increment,
            current: 0,
        }
    }

    pub fn turn_ready_to_act(&self) -> u32 {
        self.turn_ready_ap
    }
    pub fn current(&self) -> u32 {
        self.current
    }
    pub fn current_add(&mut self) -> u32 {
        self.current = self.current + self.increment;
        self.current
    }
    pub fn current_add_delta(&mut self, time: &Time) -> u32 {
        let breath = (self.increment as f32 * time.delta_seconds()) as u32;
        self.current = self.current + breath;
        self.current
    }
    pub fn current_minus(&mut self, cost: u32) -> u32 {
        self.current -= cost;
        self.current
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct HitPoints {
    max: i32,
    current: i32,
}
impl HitPoints {
    pub const DEFAULT_MAX: i32 = 1000;
    pub fn new(max: i32) -> Self {
        Self { max, current: max }
    }
    pub fn apply(&mut self, amount: i32) -> i32 {
        self.current = i32::min(self.current + amount, self.max);
        self.current
    }
}
impl Default for HitPoints {
    fn default() -> Self {
        Self {
            max: HitPoints::DEFAULT_MAX,
            current: HitPoints::DEFAULT_MAX,
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
