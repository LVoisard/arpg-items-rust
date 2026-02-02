use std::fmt::Debug;

use crate::model::requirement::RequirementBlock;
use crate::model::stat::{StatBlock, StatType};

pub trait Modifier {
    fn apply_to(&self, target: ModifierTarget);
    fn pass(&self) -> &ModifierPass;
    fn description(&self) -> String;
    fn get_affected_stat(&self) -> StatType;
}

impl Debug for Box<dyn Modifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Box").field(&self.as_ref().description()).finish()
    }
}


pub enum ModifierKind {
    Flat,
    Percent,
}

pub enum ModifierPass {
    Flat,
    Increased,
    Requirements,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ModifierTargetKind {
    Character,
    Item,
    Requirements,
}

pub enum ModifierTarget<'a> {
    Character(&'a mut StatBlock),
    Item(&'a mut StatBlock),
    Requirements(&'a mut RequirementBlock),
}

impl<'a> ModifierTarget<'a> {
    pub fn kind(&self) -> ModifierTargetKind {
        match self {
            ModifierTarget::Character(_) => ModifierTargetKind::Character,
            ModifierTarget::Item(_) => ModifierTargetKind::Item,
            ModifierTarget::Requirements(_) => ModifierTargetKind::Requirements,
        }
    }

    pub fn stats_mut(&mut self) -> Option<&mut StatBlock> {
        match self {
            ModifierTarget::Character(stats) | ModifierTarget::Item(stats) => Some(*stats),
            _ => None,
        }
    }
}

impl PartialEq<ModifierKind> for ModifierKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ModifierKind::Flat, ModifierKind::Flat) => true,
            (ModifierKind::Percent, ModifierKind::Percent) => true,
            _ => false,
        }
    }
}

impl PartialEq<ModifierPass> for ModifierPass {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ModifierPass::Flat, ModifierPass::Flat) => true,
            (ModifierPass::Increased, ModifierPass::Increased) => true,
            (ModifierPass::Requirements, ModifierPass::Requirements) => true,
            _ => false,
        }
    }
}

pub struct FlatStatModifier {
    pub value: i32,
    pub stat: StatType,
    pub target: ModifierTargetKind,
}

impl Modifier for FlatStatModifier {
    fn apply_to(&self, mut target: ModifierTarget) {
        if target.kind() != self.target {
            return;
        }

        let Some(stats) = target.stats_mut() else {
            return;
        };

        if let Some(stat) = stats.get_mut(self.stat) {
            stat.value += self.value
        }
    }

    fn pass(&self) -> &ModifierPass {
        &ModifierPass::Flat
    }

    fn description(&self) -> String {
        format!("+{} {}", self.value, self.stat)
    }

    fn get_affected_stat(&self) -> StatType {
        self.stat
    }
}
pub struct BasicStatModifier {
    pub value: i32,
    pub stat: StatType,
    pub modifier_kind: ModifierKind,
    pub modifier_pass: ModifierPass,
    pub target: ModifierTargetKind,
}

impl Modifier for BasicStatModifier {
    fn apply_to(&self, mut target: ModifierTarget) {
        if target.kind() != self.target {
            return;
        }

        let Some(stats) = target.stats_mut() else {
            return;
        };

        if let Some(stat) = stats.get_mut(self.stat) {
            match self.modifier_kind {
                ModifierKind::Flat => stat.value += self.value,
                ModifierKind::Percent => {
                    stat.value = (stat.value as f32 * (1.0 + (self.value as f32) / 100.0)) as i32
                }
            }
        }
    }

    fn pass(&self) -> &ModifierPass {
        &self.modifier_pass
    }

    fn description(&self) -> String {
        match self.modifier_kind {
            ModifierKind::Flat => format!("+{} {}", self.value, self.stat),
            ModifierKind::Percent => format!("+{}% {}", self.value, self.stat),
        }
    }

    fn get_affected_stat(&self) -> StatType {
        self.stat
    }
}

pub struct FrontStatModifier {
    pub front: StatType,
    pub value: i32,
    pub stats: Vec<StatType>,
    pub modifier_kind: ModifierKind,
    pub modifier_pass: ModifierPass,
    pub target: ModifierTargetKind,
}

impl Modifier for FrontStatModifier {
    fn apply_to(&self, mut target: ModifierTarget) {
        if target.kind() != self.target {
            return;
        }

        let Some(stats) = target.stats_mut() else {
            return;
        };

        for stat in self.stats.iter() {
            if let Some(stat) = stats.get_mut(*stat) {
                match self.modifier_kind {
                    ModifierKind::Flat => stat.value += self.value,
                    ModifierKind::Percent => {
                        stat.value =
                            (stat.value as f32 * (1.0 + (self.value as f32) / 100.0)) as i32
                    }
                }
            }
        }
    }

    fn pass(&self) -> &ModifierPass {
        &self.modifier_pass
    }

    fn description(&self) -> String {
        match self.modifier_kind {
            ModifierKind::Flat => format!("+{} {}", self.value, self.front),
            ModifierKind::Percent => format!("+{}% {}", self.value, self.front),
        }
    }

    fn get_affected_stat(&self) -> StatType {
        self.front
    }
}

pub struct CompositeStatModifier {
    pub values: Vec<i32>,
    pub stats: Vec<StatType>,
    pub modifier_kind: ModifierKind,
    pub modifier_pass: ModifierPass,
    pub target: ModifierTargetKind,
}

impl Modifier for CompositeStatModifier {
    fn apply_to(&self, mut target: ModifierTarget) {
        if target.kind() != self.target {
            return;
        }

        let Some(stats) = target.stats_mut() else {
            return;
        };

        for (index, stat) in self.stats.iter().enumerate() {
            if let Some(stat) = stats.get_mut(*stat) {
                match self.modifier_kind {
                    ModifierKind::Flat => stat.value += self.values[index],
                    ModifierKind::Percent => {
                        stat.value =
                            (stat.value as f32 * (1.0 + (self.values[index] as f32) / 100.0)) as i32
                    }
                }
            }
        }
    }

    fn pass(&self) -> &ModifierPass {
        &self.modifier_pass
    }

    fn description(&self) -> String {
        let mut res = String::new();
        for (index, stat) in self.stats.iter().enumerate() {
            match self.modifier_kind {
                ModifierKind::Flat => res.push_str(format!("+{} {}", self.values[index], stat).as_str()),
                ModifierKind::Percent => res.push_str(format!("+%{} {}", self.values[index], stat).as_str())
            }
            if index < self.stats.len() - 1 {
                res.push_str(", ");
            }
        }
        res
    }

    fn get_affected_stat(&self) -> StatType {
        self.stats[0]
    }
}


pub struct RequirementModifier {
    pub value: i32,
}

impl Modifier for RequirementModifier {
    fn apply_to(&self, target: ModifierTarget) {
        if let ModifierTarget::Requirements(reqs) = target {
            for req in reqs.requirements.iter_mut() {
                req.amount = (req.amount as f32 * (100.0 + self.value as f32) / 100.0) as i32
            }
        }
    }

    fn pass(&self) -> &ModifierPass {
        &ModifierPass::Requirements
    }

    fn description(&self) -> String {
        format!(
            "{}% {} {}",
            self.value.abs(),
            if self.value < 0 {
                "Reduced"
            } else {
                "Increased"
            },
            StatType::Requirements
        )
    }

    fn get_affected_stat(&self) -> StatType {
        StatType::Requirements
    }
}
