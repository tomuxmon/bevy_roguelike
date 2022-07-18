use bevy::{
    prelude::*,
    reflect::{FromReflect, GetTypeRegistration},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fmt::Display, hash::Hash, iter::Sum, ops::Add};
use strum::IntoEnumIterator;

pub trait AttributeType:
    Component
    + Clone
    + Eq
    + Hash
    + Display
    + Default
    + Reflect
    + FromReflect
    + GetTypeRegistration
    + IntoEnumIterator
{
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Attributes<A: AttributeType> {
    pub list: HashMap<A, u8>,
}
impl<A: AttributeType> Attributes<A> {
    pub fn get(&self, attribute_type: &A) -> u8 {
        *self.list.get(attribute_type).unwrap_or(&0)
    }
    pub fn with_all(value: u8) -> Self {
        Self {
            list: HashMap::from_iter(A::iter().map(|a| (a, value))),
        }
    }
}
impl<A: AttributeType> Default for Attributes<A> {
    fn default() -> Self {
        Self::with_all(0)
    }
}

impl<A: AttributeType> Add for Attributes<A> {
    type Output = Attributes<A>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            list: HashMap::from_iter(self.list.iter().map(|(t, v)| (t.clone(), *v + rhs.get(t)))),
        }
    }
}
impl<A: AttributeType> Sum for Attributes<A> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Attributes::default(), |acc, a| acc + a)
    }
}
impl<A: AttributeType> Display for Attributes<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.list
                .iter()
                .map(|(attribute_type, &amount)| format!("{} +{}", attribute_type, amount))
                .fold("".to_string(), |acc, x| format!("{}, {}", x, acc))
        )
    }
}
