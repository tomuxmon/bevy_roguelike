use crate::components::*;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::prelude::*;
use std::fmt::Display;
use std::ops::Range;

#[derive(Component, Clone)]
pub enum Quality {
    Broken,
    Damaged,
    Normal,
    Masterwork,
    Artifact,
}
impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Broken => write!(f, "Broken"),
            Quality::Damaged => write!(f, "Damaged"),
            Quality::Normal => write!(f, "Normal"),
            Quality::Masterwork => write!(f, "Masterwork"),
            Quality::Artifact => write!(f, "Artifact"),
        }
    }
}
impl Quality {
    pub fn roll(rng: &mut StdRng) -> Self {
        // Broken 5 %
        // Damaged 20 %
        // Normal 50%
        // Masterwork 20 %
        // Artifact 5 %
        match rng.gen_range(0..100) {
            0..=5 => Self::Broken,
            6..=25 => Self::Damaged,
            26..=75 => Self::Normal,
            76..=95 => Self::Masterwork,
            96..=100 => Self::Artifact,
            _ => Self::Normal,
        }
    }
    /// broken (20% .. 60%),
    /// damaged (60% .. 90%),
    /// normal (90 .. 110%),
    /// masterwork (110% .. 140%),
    /// artifact (140% .. 200%)
    pub fn get_multiplier(&self) -> Range<u8> {
        match self {
            Quality::Broken => 20..60,
            Quality::Damaged => 60..90,
            Quality::Normal => 90..110,
            Quality::Masterwork => 110..140,
            Quality::Artifact => 140..200,
        }
    }
}
impl Default for Quality {
    fn default() -> Self {
        Self::Normal
    }
}

pub trait MutableQuality {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self;
}
impl MutableQuality for u8 {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        let Range { start, end } = quality.get_multiplier();
        let t_start = (*self as f32 * start as f32 / 100 as f32) as u8;
        let t_end = (*self as f32 * end as f32 / 100 as f32) as u8;
        let range = t_start..t_end;
        if range.is_empty() {
            range.start
        } else {
            rng.gen_range(range)
        }
    }
}
impl MutableQuality for i16 {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        let Range { start, end } = quality.get_multiplier();
        let t_start = (*self as f32 * start as f32 / 100 as f32) as i16;
        let t_end = (*self as f32 * end as f32 / 100 as f32) as i16;
        let range = t_start..t_end;
        if range.is_empty() {
            range.start
        } else {
            rng.gen_range(range)
        }
    }
}
impl MutableQuality for i32 {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        let Range { start, end } = quality.get_multiplier();
        let t_start = (*self as f32 * start as f32 / 100 as f32) as i32;
        let t_end = (*self as f32 * end as f32 / 100 as f32) as i32;
        let range = t_start..t_end;
        if range.is_empty() {
            range.start
        } else {
            rng.gen_range(range)
        }
    }
}
impl MutableQuality for Range<i32> {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        let Range { start, end } = *self;
        let start_new = start.mutate(quality, rng);
        let end_new = end.mutate(quality, rng);
        if start_new > end_new {
            Range {
                start: end_new,
                end: start_new,
            }
        } else {
            Range {
                start: start_new,
                end: end_new,
            }
        }
    }
}
impl MutableQuality for AttributeMultiplier {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            multiplier: self.multiplier.mutate(quality, rng),
            attribute: self.attribute,
        }
    }
}
impl MutableQuality for Formula {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            multipliers: HashSet::from_iter(
                self.multipliers.iter().map(|m| m.mutate(quality, rng)),
            ),
        }
    }
}
impl MutableQuality for Option<Formula> {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        if let Some(f) = self {
            Some(f.mutate(quality, rng))
        } else {
            None
        }
    }
}

impl MutableQuality for Rate {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            amount: self.amount.mutate(quality, rng),
            multiplier: self.multiplier.mutate(quality, rng),
        }
    }
}
impl MutableQuality for ActionCost {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            cost: self.cost.mutate(quality, rng),
            multiplier_inverted: self.multiplier_inverted.mutate(quality, rng),
        }
    }
}
impl MutableQuality for Damage {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            kind: self.kind,
            amount: self.amount.mutate(quality, rng),
            amount_multiplier: self.amount_multiplier.mutate(quality, rng),
            hit_cost: self.hit_cost.mutate(quality, rng),
            hit_chance: self.hit_chance.mutate(quality, rng),
        }
    }
}
impl MutableQuality for Protect {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            kind: self.kind,
            amount_multiplier: self.amount_multiplier.mutate(quality, rng),
            amount: self.amount.mutate(quality, rng),
        }
    }
}
impl MutableQuality for Protection {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            amounts: self
                .amounts
                .iter()
                .map(|a| a.mutate(quality, rng))
                .collect(),
        }
    }
}
impl MutableQuality for Resist {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            kind: self.kind,
            percent: self.percent.mutate(quality, rng),
        }
    }
}
impl MutableQuality for Resistance {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            amounts: HashSet::from_iter(self.amounts.iter().map(|p| p.mutate(quality, rng))),
        }
    }
}
impl MutableQuality for Evasion {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            cost: self.cost.mutate(quality, rng),
            chance: self.chance.mutate(quality, rng),
        }
    }
}
impl MutableQuality for Block {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            block_type: self.block_type.clone(),
            cost: self.cost.mutate(quality, rng),
            chance: self.chance.mutate(quality, rng),
        }
    }
}
impl MutableQuality for Attributes {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            strength: self.strength.mutate(quality, rng),
            dexterity: self.dexterity.mutate(quality, rng),
            inteligence: self.inteligence.mutate(quality, rng),
            toughness: self.toughness.mutate(quality, rng),
            perception: self.perception.mutate(quality, rng),
            willpower: self.willpower.mutate(quality, rng),
        }
    }
}
