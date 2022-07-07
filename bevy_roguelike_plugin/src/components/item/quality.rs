use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_roguelike_combat::stats_derived::*;
use bevy_roguelike_combat::*;
use rand::prelude::*;
use std::fmt::Display;
use std::ops::Range;

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
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
    /// broken (140% .. 200%),
    /// damaged (110% .. 140%),
    /// normal (90 .. 110%),
    /// masterwork (60% .. 90%),
    /// artifact (20% .. 60%),
    pub fn get_multiplier_inverse(&self) -> Range<u8> {
        match self {
            Quality::Broken => 140..200,
            Quality::Damaged => 110..140,
            Quality::Normal => 90..110,
            Quality::Masterwork => 60..90,
            Quality::Artifact => 20..60,
        }
    }
}
impl Default for Quality {
    fn default() -> Self {
        Self::Normal
    }
}

pub trait MutableQuality {
    fn mutate(&self, quality: &Quality, rng: &mut StdRng) -> Self
    where
        Self: Sized,
    {
        self.mutate_extended(true, quality, rng)
    }
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self;
}
impl MutableQuality for u8 {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        let Range { start, end } = if is_direct {
            quality.get_multiplier()
        } else {
            quality.get_multiplier_inverse()
        };
        let t_start = (*self as f32 * start as f32 / 100.) as u8;
        let t_end = (*self as f32 * end as f32 / 100.) as u8;
        let range = t_start..t_end;
        if range.is_empty() {
            range.start
        } else {
            rng.gen_range(range)
        }
    }
}
impl MutableQuality for i16 {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        let Range { start, end } = if is_direct {
            quality.get_multiplier()
        } else {
            quality.get_multiplier_inverse()
        };
        let t_start = (*self as f32 * start as f32 / 100.) as i16;
        let t_end = (*self as f32 * end as f32 / 100.) as i16;
        let range = t_start..t_end;
        if range.is_empty() {
            range.start
        } else {
            rng.gen_range(range)
        }
    }
}
impl MutableQuality for i32 {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        let Range { start, end } = if is_direct {
            quality.get_multiplier()
        } else {
            quality.get_multiplier_inverse()
        };
        let t_start = (*self as f32 * start as f32 / 100.) as i32;
        let t_end = (*self as f32 * end as f32 / 100.) as i32;
        let range = t_start..t_end;
        if range.is_empty() {
            range.start
        } else {
            rng.gen_range(range)
        }
    }
}
impl MutableQuality for Range<i32> {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        let Range { start, end } = *self;
        let start_new = start.mutate_extended(is_direct, quality, rng);
        let end_new = end.mutate_extended(is_direct, quality, rng);
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
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            multiplier: self.multiplier.mutate_extended(is_direct, quality, rng),
            attribute: self.attribute,
        }
    }
}
impl MutableQuality for Formula {
    fn mutate_extended(&self, inverted: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            multipliers: HashSet::from_iter(
                self.multipliers
                    .iter()
                    .map(|m| m.mutate_extended(inverted, quality, rng)),
            ),
        }
    }
}
impl MutableQuality for Option<Formula> {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        self.as_ref()
            .map(|f| f.mutate_extended(is_direct, quality, rng))
    }
}

impl MutableQuality for Rate {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            amount: self.amount.mutate_extended(is_direct, quality, rng),
            multiplier: self.multiplier.mutate_extended(is_direct, quality, rng),
        }
    }
}
impl MutableQuality for ActionCost {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            cost: self.cost.mutate_extended(!is_direct, quality, rng),
            multiplier_inverted: self
                .multiplier_inverted
                .mutate_extended(!is_direct, quality, rng),
        }
    }
}
impl MutableQuality for Damage {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            kind: self.kind,
            amount: self.amount.mutate_extended(is_direct, quality, rng),
            amount_multiplier: self
                .amount_multiplier
                .mutate_extended(is_direct, quality, rng),
            hit_cost: self.hit_cost.mutate_extended(is_direct, quality, rng),
            hit_chance: self.hit_chance.mutate_extended(is_direct, quality, rng),
        }
    }
}
impl MutableQuality for Protect {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            kind: self.kind,
            amount_multiplier: self
                .amount_multiplier
                .mutate_extended(is_direct, quality, rng),
            amount: self.amount.mutate_extended(is_direct, quality, rng),
        }
    }
}
impl MutableQuality for Protection {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            amounts: self
                .amounts
                .iter()
                .filter_map(|a| {
                    let protect = a.mutate_extended(is_direct, quality, rng);
                    if protect.amount > 0 {
                        Some(protect)
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}
impl MutableQuality for Resist {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            kind: self.kind,
            percent: self.percent.mutate_extended(is_direct, quality, rng),
        }
    }
}
impl MutableQuality for Resistance {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            amounts: HashSet::from_iter(self.amounts.iter().filter_map(|p| {
                let aa = p.mutate_extended(is_direct, quality, rng);
                if aa.percent > 0 {
                    Some(aa)
                } else {
                    None
                }
            })),
        }
    }
}
impl MutableQuality for Evasion {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            cost: self.cost.mutate_extended(is_direct, quality, rng),
            chance: self.chance.mutate_extended(is_direct, quality, rng),
        }
    }
}
impl MutableQuality for Block {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            block_type: self.block_type.clone(),
            cost: self.cost.mutate_extended(is_direct, quality, rng),
            chance: self.chance.mutate_extended(is_direct, quality, rng),
        }
    }
}
impl MutableQuality for Attributes {
    fn mutate_extended(&self, is_direct: bool, quality: &Quality, rng: &mut StdRng) -> Self {
        Self {
            list: HashMap::from_iter(self.clone().list.into_iter().filter_map(|(t, v)| {
                let attribute = (t, v.mutate_extended(is_direct, quality, rng));
                if attribute.1 > 0 {
                    Some(attribute)
                } else {
                    None
                }
            })),
        }
    }
}
