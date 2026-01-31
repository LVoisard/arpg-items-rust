use uuid::Uuid;

use crate::model::item_builder::ItemBuilder;
use crate::model::modifier::{Modifier, ModifierPass, ModifierTarget};
use crate::model::requirement::{RequirementBlock};
use crate::model::stat::{StatBlock};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Item {
    pub id: Uuid,
    pub item_base: String,
    pub name: Option<String>,
    pub rarity: ItemRarity,
    pub item_class: ItemClass,
    pub requirements: RequirementBlock,
    pub base_stats: StatBlock,
    pub modifiers: Vec<Box<dyn Modifier>>,
}

impl Item {
    pub fn builder() -> ItemBuilder {
        ItemBuilder::new()
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
}

#[derive(Debug, Copy,Clone)]
pub enum ItemRarity {
    Normal,
    Magic,
    Rare,
    Unique,
}

#[derive(Debug, Clone)]
pub enum ItemClass {
    Equipment(EquipmentType),
    None
}

impl Display for ItemClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemClass::Equipment(equipment_type) => write!(f, "{}", equipment_type.to_string()),
            ItemClass::None => write!(f, "{}", "None")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug,Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
