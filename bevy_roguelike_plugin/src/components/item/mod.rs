use super::RenderInfo;
use bevy::{prelude::*, utils::HashSet};
use std::borrow::Cow;
use std::iter::Sum;
use std::ops::{Deref, DerefMut};

#[derive(Bundle)]
pub struct Weapon {
    item: Item,
    name: Name,
    attack: AttackBoost,
    render_info: RenderInfo,
    // Vector2D should be added separately if item is to be placed on the map
}

impl Weapon {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        attack: AttackBoost,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            item: Item {},
            name: Name::new(name),
            attack,
            render_info: RenderInfo { texture, z: 1. },
        }
    }
}

#[derive(Bundle)]
pub struct Armor {
    item: Item,
    name: Name,
    attack: DefenseBoost,
    render_info: RenderInfo,
    //Vector2D should be added separately if item is to be placed on the map
}
impl Armor {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        attack: DefenseBoost,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            item: Item {},
            name: Name::new(name),
            attack,
            render_info: RenderInfo { texture, z: 1. },
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Item;

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct AttackBoost {
    damage: i16,
    rate: i16,
    cost: i16,
}
impl AttackBoost {
    pub fn new(damage: i16, rate: i16, cost: i16) -> Self {
        Self { damage, rate, cost }
    }
    pub fn damage(&self) -> i16 {
        self.damage
    }
    pub fn rate(&self) -> i16 {
        self.rate
    }
    pub fn cost(&self) -> i16 {
        self.cost
    }
}
impl<'a> Sum<&'a Self> for AttackBoost {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(AttackBoost::new(0, 0, 0), |a, b| {
            Self::new(a.damage + b.damage, a.rate + b.rate, a.cost + b.cost)
        })
    }
}

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct DefenseBoost {
    absorb: i16,
    rate: i16,
    cost: i16,
}
impl DefenseBoost {
    pub fn new(absorb: i16, rate: i16, cost: i16) -> Self {
        Self { absorb, rate, cost }
    }
    pub fn absorb(&self) -> i16 {
        self.absorb
    }
    pub fn rate(&self) -> i16 {
        self.rate
    }
    pub fn cost(&self) -> i16 {
        self.cost
    }
}
impl<'a> Sum<&'a Self> for DefenseBoost {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(DefenseBoost::new(0, 0, 0), |a, b| {
            Self::new(a.absorb + b.absorb, a.rate + b.rate, a.cost + b.cost)
        })
    }
}

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Equiped;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    items: HashSet<Entity>,
}
impl Deref for Inventory {
    type Target = HashSet<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl DerefMut for Inventory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
