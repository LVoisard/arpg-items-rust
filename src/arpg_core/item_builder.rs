use std::fmt::{Display, Formatter};
use uuid::{Uuid, uuid};

use crate::arpg_core::item::{Item, ItemClass, ItemRarity};
use crate::arpg_core::modifier::Modifier;
use crate::arpg_core::requirement::{RequirementBlock, StatRequirement};
use crate::arpg_core::stat::{Stat, StatBlock, StatType};

#[derive(Debug)]
pub struct ItemCreationError(String);

impl Display for ItemCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Item creation error: {}", self.0)
    }
}

pub struct ItemBuilder {
    name: Option<String>,
    item_base: Option<String>,
    rarity: ItemRarity,
    item_class: ItemClass,
    requirements: RequirementBlock,
    base_stats: StatBlock,
    modifiers: Vec<Box<dyn Modifier>>,
}

impl Default for ItemBuilder {
    fn default() -> Self {
        ItemBuilder {
            name: None,
            item_base: None,
            rarity: ItemRarity::Normal,
            item_class: ItemClass::None,
            requirements: RequirementBlock::default(),
            base_stats: StatBlock::default(),
            modifiers: Vec::new(),
        }
    }
}

impl ItemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn base(mut self, base: String) -> Self {
        self.item_base = Some(base);
        self
    }

    pub fn class(mut self, class: ItemClass) -> Self {
        self.item_class = class;
        self
    }

    pub fn rarity(mut self, rarity: ItemRarity) -> Self {
        self.rarity = rarity;
        self
    }

    pub fn with_stat(mut self, stat_type: StatType, value: i32) -> Self {
        self.base_stats.stats.push(Stat{
            stat_type,
            value,
        });
        self
    }

    pub fn with_modifier(mut self, modifier: impl Modifier + 'static) -> Self {
        self.modifiers.push(Box::new(modifier));
        self
    }

    pub fn with_requirement(mut self, stat_type: StatType, value: i32) -> Self {
        self.requirements.requirements.push(StatRequirement {
            stat_type,
            amount: value,
        });
        self
    }

    pub fn build(self) -> Result<Item, ItemCreationError> {
        let item = Item {
            id: Uuid::new_v4(),
            item_base: match self.item_base {
                None => return Err(ItemCreationError(String::from("Item Base not specified"))),
                Some(base) => base,
            },
            name: self.name,
            rarity: self.rarity,
            item_class: self.item_class,
            requirements: self.requirements,
            base_stats: self.base_stats,
            modifiers: self.modifiers,
        };

        Ok(item)
    }
}
