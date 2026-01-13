use crate::arpg_core::modifier::{Modifier, ModifierKind, ModifierPass, ModifierTarget};
use crate::arpg_core::requirement::{RequirementBlock, StatRequirement};
use crate::arpg_core::stat::{StatBlock, StatType};
use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Item {
    pub item_base: String,
    pub name: Option<String>,
    pub rarity: ItemRarity,
    pub item_class: ItemClass,
    pub requirements: RequirementBlock,
    pub base_stats: StatBlock,
    pub modifiers: Vec<Box<dyn Modifier>>,
}

pub struct ItemPresentation {
    pub item_base: String,
    pub name: Option<String>,
    pub rarity: ItemRarity,

    pub damage: Option<DamageLine>,
    pub requirements: Vec<RequirementLine>,
    pub modifiers: Vec<String>,
}

pub struct DamageLine {
    pub min: i32,
    pub max: i32,
    pub is_modified: bool,
}

pub struct RequirementLine {
    pub requirement: StatRequirement,
    pub is_met: bool,
}

impl Debug for Box<dyn Modifier> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl Item {
    pub fn get_derived_stats(&self) -> StatBlock {
        let mut stats = self.base_stats.clone();

        for pass in [ModifierPass::Flat, ModifierPass::Increased] {
            for modifier in self.modifiers.iter().filter(| x | *x.pass() == pass) {
                match pass {
                    ModifierPass::Requirements => {}
                    _ => modifier.apply_to(ModifierTarget::Item(&mut stats)),
                }
            }
        }

        stats
    }

    pub fn get_derived_requirements(&self) -> RequirementBlock {
        let mut reqs = self.requirements.clone();

        for modifier in self.modifiers.iter().filter(| x | *x.pass() == ModifierPass::Requirements) {
                modifier.apply_to(ModifierTarget::Requirements(&mut reqs))

        }

        reqs
    }

    pub fn present(&self, player_stats: &StatBlock) -> ItemPresentation {
        let derived = self.get_derived_stats();
        let reqs = self.get_derived_requirements();

        let damage = match self.item_class {
            ItemClass::Equipment(EquipmentType::Weapon(_))
            if derived.has(StatType::MinimumDamage)
                && derived.has(StatType::MaximumDamage) =>
                {
                    Some(DamageLine {
                        min: derived.get(StatType::MinimumDamage).unwrap().value,
                        max: derived.get(StatType::MaximumDamage).unwrap().value,
                        is_modified: self.modifiers.iter().any(|m|
                            matches!(
                            m.get_affected_stat(),
                            StatType::IncreasedDamage
                                | StatType::MinimumDamage
                                | StatType::MaximumDamage
                        )
                        ),
                    })
                }
            _ => None,
        };

        let requirements = reqs.requirements.into_iter().map(|r| {
            let met = player_stats
                .get(r.stat_type)
                .map(|s| r.is_met(s))
                .unwrap_or(false);

            RequirementLine {
                requirement: r,
                is_met: met,
            }
        }).collect();

        ItemPresentation {
            name: self.name.clone(),
            item_base: self.item_base.clone(),
            rarity: self.rarity,
            damage,
            requirements,
            modifiers: self.modifiers.iter().map(|m| m.description()).collect(),
        }
    }
}

#[derive(Debug, Copy,Clone)]
pub enum ItemRarity {
    Normal,
    Magic,
    Rare,
    Unique,
}

#[derive(Debug)]
pub enum ItemClass {
    Equipment(EquipmentType),
}

impl Display for ItemClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemClass::Equipment(equipment_type) => write!(f, "{}", equipment_type.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum EquipmentType {
    Armour(ArmourType),
    Weapon(WeaponType),
    Jewellery(JewelleryType),
}

impl Display for EquipmentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EquipmentType::Armour(t) => write!(f, "{}", t.to_string()),
            EquipmentType::Weapon(t) => write!(f, "{}", t.to_string()),
            EquipmentType::Jewellery(t) => write!(f, "{}", t.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum ArmourType {
    Helmet,
    BodyArmour,
    Gloves,
    Boots,
    Shield,
}

impl Display for ArmourType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmourType::Helmet => write!(f, "Helmet"),
            ArmourType::BodyArmour => write!(f, "Body Armour"),
            ArmourType::Gloves => write!(f, "Gloves"),
            ArmourType::Boots => write!(f, "Boots"),
            ArmourType::Shield => write!(f, "Shield"),
        }
    }
}

#[derive(Debug)]
pub enum WeaponType {
    Sword,
    Dagger,
    Axe,
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponType::Sword => write!(f, "Sword"),
            WeaponType::Dagger => write!(f, "Dagger"),
            WeaponType::Axe => write!(f, "Axe"),
        }
    }
}

#[derive(Debug)]
pub enum JewelleryType {
    Belt,
    Ring,
    Amulet,
}

impl Display for JewelleryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JewelleryType::Belt => write!(f, "Belt"),
            JewelleryType::Ring => write!(f, "Ring"),
            JewelleryType::Amulet => write!(f, "Amulet"),
        }
    }
}
